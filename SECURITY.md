# Security Policy

injection-scanner parses attacker-controlled input (skill files, CLAUDE.md, RAG documents, user
submissions) and its output is used to gate commits and CI pipelines. That gives it two different
security surfaces, and this document covers both.

## Supported versions

`v0.0.3` is the current release and the only supported line. Fixes ship as a new tag; there are no
backports to `v0.0.1` or `v0.0.2`. `main` is ahead of `v0.0.3` with unreleased behaviour and is not
a supported release — if you are running a build from `main`, expect to re-verify against the next
tagged release before relying on a fix.

This is a single-maintainer project ([@hermanngeorge15](https://github.com/hermanngeorge15)). Set
expectations accordingly — see "What to expect" below.

## Reporting a vulnerability in the scanner

Report privately via GitHub's private vulnerability reporting on this repository:

<https://github.com/UnityInFlow/injection-scanner/security/advisories/new>

There is no separate email address or contact channel for security reports — use the advisory form.

In scope, concretely, because the scanner reads files it does not control:

- A panic or crash triggered by a crafted input file.
- Catastrophic regex backtracking (ReDoS) in a library pattern, or anything that turns a scan into
  a hang.
- Path traversal, or a write outside the tree being scanned.
- Memory or CPU exhaustion from a hostile file.
- Anything that lets the **content of a scanned file** affect the host running the scan, or affect
  a CI job's outcome beyond the documented exit codes (`0` / `1` / `2`).

Use the advisory form for any of the above rather than a public issue — a working proof-of-concept
for a crash or a ReDoS is itself the exploit.

## Detection bypasses are NOT embargoed vulnerabilities

This is the other half, and it needs to be unambiguous: **a payload the scanner misses is a public
issue, not a security advisory.**

The pattern library is known-incomplete, and that is tracked in the open, not embargoed:

- [#80](https://github.com/UnityInFlow/injection-scanner/issues/80) — role-override patterns are
  near-literal; common synonyms defeat every one.
- [#81](https://github.com/UnityInFlow/injection-scanner/issues/81) — nothing measures recall; the
  clean corpus gates false positives, but no gate exists for missed attacks.
- [`docs/DETECTION-BACKLOG.md`](docs/DETECTION-BACKLOG.md) — the standing list of what the library
  does not yet cover.

The reason is simple: a bypass that is embargoed is a bypass nobody can write a regression test
for. If you find a phrasing, an encoding, or a technique that gets past the scanner, open it as a
public issue using the **pattern proposal** form so it can be turned into a test case and a fix
like any other missing detection.

## Also not vulnerabilities

- **Inline suppression by an untrusted document.** This is a documented trust boundary, not a
  defect — see "Suppression is a trust boundary" in [`README.md`](README.md). Whoever can edit the
  file being scanned can suppress findings in it; the scanner's answer is that suppression is never
  silent, and `--no-suppress` exists for exactly the case where you did not write the file.
- **A false positive.** Use the `false_positive.yml` issue form, not an advisory.

## What to expect

For a single maintainer working on this part-time: acknowledgement within 7 days, an initial
assessment within 14 days, best effort after that. There is no guaranteed remediation window, no
bug bounty, and no monetary reward. Credit is offered in the advisory and in `CHANGELOG.md` unless
you decline it.

## Supply chain

Every release binary carries a signed SLSA build-provenance attestation, verifiable with:

```bash
gh attestation verify injection-scanner-x86_64-unknown-linux-musl --repo UnityInFlow/injection-scanner
```

Each release also publishes `SHA256SUMS.txt` alongside the binaries.

A report of a **tampered release asset** — a binary that does not match its published checksum or
attestation — is in scope and urgent: `spec-ci-plugin` downloads and executes these binaries
directly in other repositories' CI, so a compromised asset here is a compromise there. Use the
advisory form for this, same as any other vulnerability report.
