# ADR-003: SARIF Output

Date: 2026-08-25
Status: Accepted

## Context

`--format sarif` was one of the original v0.0.1 promises (issue #5, requirement CLI-04)
and, until this change, silently returned human-readable text with a findings exit
code — one of the findings in the 2026-08 audit that opened this milestone. The
`OutputFormat` enum in `src/main.rs` explicitly withheld a `Sarif` variant with a doc
comment forbidding it before a real writer existed, to stop that exact silent failure
recurring.

This ADR is required by `pr-artifacts`'s explicit trigger: "output format contracts
(JSON, SARIF)".

Several design questions had non-obvious answers, each with a real failure mode on
the wrong side:

- Which findings become SARIF results, when the report already carries three
  withheld arrays (`suppressed`, `low_confidence`, `baselined`) alongside `matches`?
- SARIF's `level` has three useful slots (`error`/`warning`/`note`); this tool has
  four severities. Where does the fourth go?
- SARIF's `partialFingerprints` exist to keep an alert identified across runs. Can
  the existing `baseline::fingerprint` (sha256 over `matched_text`, CLI-08/ADR-002)
  be reused directly?
- How is a SARIF document validated, given this repo has no network access in CI and
  scans itself?
- Where does `security-events: write` live, given `ci.yml` runs on `pull_request`
  and must never hold a write scope?
- This repository's own attack corpora (`examples/`, `patterns/core/*.yaml`,
  `tests/fixtures/`) are agent-facing content by design. Uploading them unfiltered
  floods the Security tab; excluding them creates a blind spot exactly where a
  malicious pattern contribution would land.

## Decision

**D-1 — only `ScanReport.matches` become SARIF results.** `suppressed`,
`low_confidence` and `baselined` are never read by the writer
(`src/sarif.rs::format_sarif`). A code-scanning alert means "act on this"; the three
withheld arrays each mean the opposite, for three different reasons, and they
already have a home — `--format json`, where a consumer wanting the full picture
looks. SARIF's `suppressions` property (per-result, marking a result as dismissed)
was considered for the `baselined` case specifically and rejected: it would put an
already-accepted finding in front of reviewers as a dismissed alert, which is noise
in exactly the workflow `--baseline` exists to quiet. Enforced at the single place
the results array is built — `format_sarif` takes `&[ScanReport]` and reads
`report.matches` — not by filtering a superset downstream, so there is no second
code path that could disagree with D-1.

**Severity — `level` collapses 4-to-3; `properties["security-severity"]` on the
rule, not `rank`, recovers it.** CRITICAL and HIGH both map to `error`, MEDIUM to
`warning`, LOW to `note` (`level_for`, exhaustive over `Severity`). `rank` (SARIF's
own numeric ranking property) was proposed and rejected: GitHub's code-scanning
ingest does not read it. What it reads is `properties["security-severity"]` on the
**rule descriptor** — a `"0.0"`-`"10.0"` string it bands into the Security tab's
displayed severity (critical ≥9.0, high 7.0-8.9, medium 4.0-6.9, low <4.0) — plus
the literal `security` tag, without which GitHub ignores `security-severity`
entirely. `rank` would have been a second, differently-scaled severity number that
nothing consumes and that could silently drift out of step with `level`. Every
result also carries `properties.severity` with the native string, for any consumer
that is not GitHub. One value per native severity, chosen comfortably inside its
GitHub band: `9.0`/`7.0`/`5.0`/`2.0` for CRITICAL/HIGH/MEDIUM/LOW.

**Fingerprints — reuse `baseline::fingerprint`, promoted to `pub`, plus an
occurrence ordinal.** `fingerprint` (`src/baseline.rs`) was not already public,
contrary to earlier assumptions recorded in this milestone's planning; promoting it
was part of this change. The bare digest — sha256 over `matched_text` alone — is
**not** a sufficient `partialFingerprint` on its own: it hashes neither the file nor
the line, so two occurrences of the identical payload in one file produce
byte-identical `(ruleId, uri, partialFingerprint)` triples, the tuple GitHub tracks
an alert by. The two would collapse into one alert, and fixing one occurrence would
then close an alert whose twin payload is still live — a missed alert, which is the
wrong failure direction for a security scanner. The fix: key
`matchedTextSha256/v1`, value `<fingerprint>/<n>`, where `n` is the 1-based ordinal
of this occurrence within its `(file, ruleId, digest)` group — the same grouping
`Baseline::from_reports` already counts. Always suffixed, including the first
occurrence, so there is one rule and no special case. Verified: two identical
payloads in one file get two distinct SARIF identities, and prepending ten blank
lines to the file leaves both fingerprints unchanged (line-number independence is
preserved, which is the property the reuse exists for). Accepted cost: deleting the
first of two identical payloads renumbers the survivor, so GitHub closes one alert
and opens what is functionally a new one for the same payload — cosmetic churn,
traded against a silently missed alert.

**Validation — structural assertions plus the real upload; no vendored schema, no
new dev-dependency.** `jsonschema` was rejected: it pulls a large dependency tree
(`fancy-regex`, `url`, `referencing` and others) into the `Cargo.lock` of a tool
that ships SLSA-attested binaries, SHA-pins its GitHub Actions, and has kept itself
to a dozen runtime dependencies — one test assertion does not buy that. Vendoring
`sarif-schema-2.1.0.json` was also rejected: the schema is roughly half a megabyte,
`json` is in `DEFAULT_EXTENSIONS`, and this repo scans itself, so every `check .`
and every pre-commit run would walk it — for a check that is necessary but not
sufficient anyway, since a document can be schema-valid and still be rejected or
mis-rendered by GitHub's actual ingest. Instead: the writer is built from
`#[derive(Serialize)]` structs, fixing the document shape at compile time rather
than assembling it ad hoc; `tests/sarif_test.rs` asserts SARIF 2.1.0's required
properties explicitly (`$schema`, `version`, `runs`, `tool.driver.name`,
`message.text`, `artifactLocation.uri`, `region.startLine >= 1`, the closed `level`
set); and the real end-to-end proof is `.github/workflows/code-scanning.yml`
uploading to GitHub's own ingest, which validates against 2.1.0 and rejects invalid
documents — the actual point where "validates against the schema" is settled.

**`ci.yml` is not modified; `security-events: write` lives in
`.github/workflows/code-scanning.yml` alone.** `ci.yml` runs on `pull_request`, so
it executes fork-authored build scripts and proc macros. Granting a write scope to
a job that runs attacker-supplied code is the exact escalation `ci.yml`'s own
RUNNER POLICY header forbids. The new workflow triggers only on `push: main`, a
weekly `schedule`, and `workflow_dispatch` — none of which a fork PR can fire —
mirroring the shape `release.yml` already uses for its own elevated permissions.
`ci.yml`'s diff is asserted empty by an automated check
(`git diff --exit-code .github/workflows/ci.yml`), so "the fork-facing workflow
gained no scope" is a verifiable claim, not an assertion to trust.

**Self-scan noise — a committed baseline, not blanket excludes.** Uploading this
repo's own findings unfiltered would put 51 alerts of pure noise (attack corpora in
`examples/`, payload-shaped regexes in `patterns/core/*.yaml`, `tests/fixtures/`,
`tools/injection-lab/corpus/`) on the public Security tab on day one. Blanket
`--exclude 'examples/**' --exclude 'patterns/**'` would silence the noise but build
a permanent blind spot into exactly the two directories a malicious community
pattern PR would land in. Instead: `.github/code-scanning-baseline.json` is
committed (51 entries, generated by
`check . --exclude '.planning/**' --write-baseline ...`), and the workflow scans
with `--baseline`. Verified on this checkout: the rescan with the baseline applied
reports zero findings and exits 0, and the baseline file itself scans clean (it
stores `sha256:<hex>` digests, never the payload — ADR-002). D-1 does the rest: a
baselined finding never becomes a SARIF result, so a clean `main` uploads an empty
`results` array while any genuinely new payload anywhere in the repository — new or
existing directory — still becomes an alert. `.planning/**` is excluded by glob
rather than baselined: it is planning prose, not shipped or agent-facing surface,
and grows a new payload-describing document on every quick task, which would make
it pure baseline churn rather than a durable accept decision.

**`rules` keeps its own format enum (`RulesFormat`), not `OutputFormat`.** A
`rules --format sarif` document would have `tool.driver.rules` populated from the
full pattern catalogue and `results: []` — which reads to a code-scanning consumer
not as "here is the rule catalogue" but as "this analysis found nothing", and
uploading it would close every open alert in every category it lists (T-uor-04).
`Commands::Rules` now takes its own `RulesFormat { Text, Json }`, so clap continues
rejecting `--format sarif` on `rules` at parse time with the valid list — the exact
property the original `OutputFormat` doc comment existed to protect, preserved
rather than traded away for a single shared enum.

## Consequences

### Positive

- A code-scanning alert means exactly what the CLI's own exit code already means —
  "here is a finding to act on" — with no separate mental model for what SARIF
  reports versus what `--format json` reports.
- The native four-level severity is never actually lost: it is recoverable from
  `result.properties.severity` for any consumer, and distinguishable in GitHub's own
  Security tab via `security-severity`, which `level` alone could not provide.
- The baseline and the SARIF alert identity are now the same code path
  (`baseline::fingerprint`), so a change to what "the same finding" means moves both
  consumers together instead of drifting into two definitions that disagree.
- `tool.driver.rules` lists every loaded pattern regardless of whether it fired,
  which makes the SARIF document useful as a rule catalogue on its own, and means a
  `ruleIndex` can never dangle: the rule set is always a superset of what can
  produce a `ScanMatch` (`Scanner::new_lenient` only ever drops patterns).
- No new runtime or dev dependency, no vendored schema, no self-hosted runner, and
  `ci.yml`'s diff stays empty and machine-verified.

### Negative / Trade-offs

- SARIF validity is asserted structurally plus by the real upload succeeding, not by
  a local schema check — a malformed document could in principle still pass local
  tests and only fail at GitHub's ingest, days after `main` last changed (mitigated
  by the weekly `schedule` trigger surfacing this even on a quiet repo).
- Deleting one of two identical payloads renumbers the fingerprint ordinal of the
  survivor, so GitHub sees "one alert closed, one new alert opened" rather than "one
  alert closed, one persisted" — a cosmetic discontinuity, traded deliberately
  against the missed-alert failure mode the ordinal exists to prevent.
- The committed baseline must be regenerated by hand whenever `examples/`,
  `patterns/`, or the corpus under `tools/injection-lab/` changes intentionally, or
  new baseline noise appears as apparent alerts (in practice: rescanned as stale
  entries reported on stderr, not as false alarms, but still a manual step).

## Alternatives Considered

- **SARIF `suppressions` for baselined findings.** Rejected under D-1 — it would
  surface an accepted finding to reviewers as a dismissed alert, defeating the
  quiet-adoption purpose of `--baseline`.
- **`rank` for carrying native severity.** Rejected — GitHub's ingest does not
  consume it; `properties["security-severity"]` on the rule descriptor does, and is
  the only mechanism that actually changes what a reviewer sees.
- **`jsonschema` crate + a runtime schema check.** Rejected — brings a large
  transitive dependency tree into a 12-dependency security tool for one assertion
  that a real upload proves more completely.
- **A vendored `sarif-schema-2.1.0.json`.** Rejected — roughly 500KB, and `json` is
  a scanned extension by default, so this repo's own self-scan would walk it on
  every run for a check that is necessary but not sufficient.
- **Blanket `--exclude` on `examples/`/`patterns/`.** Rejected — silences the noise
  by creating a permanent blind spot in exactly the directories a malicious
  community pattern PR would target.
- **A shared `OutputFormat` covering `rules` too.** Rejected — a rules-only SARIF
  document with an empty `results` array is indistinguishable from "nothing found"
  to a code-scanning consumer and would close every open alert in its categories.
