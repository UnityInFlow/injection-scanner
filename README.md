# injection-scanner

Prompt injection is the SQL injection of the AI era. As AI agents process untrusted text -- skill files, RAG documents, user inputs, CLAUDE.md specs -- a single injected instruction can hijack agent behavior, exfiltrate data, or bypass safety controls.

**injection-scanner** is a static analysis tool that catches prompt injection patterns before they reach your AI agent. It scans files for role overrides, instruction injection, data exfiltration, jailbreaks, and encoding attacks using a maintained YAML pattern library.

## Installation

Download the pre-built binary for your platform from [GitHub Releases](https://github.com/UnityInFlow/injection-scanner/releases).

```bash
# macOS / Linux
chmod +x injection-scanner
sudo mv injection-scanner /usr/local/bin/

# Verify
injection-scanner --version
```

Or build from source:

```bash
cargo install --path .
```

## Usage

### Scan a single file

```bash
injection-scanner check CLAUDE.md
```

### Scan a directory recursively

```bash
injection-scanner check .
```

### Read from stdin

```bash
cat skill.md | injection-scanner check -
```

### JSON output

```bash
injection-scanner check CLAUDE.md --format json
```

### Controlling what gets walked

`check .` honours `.gitignore` and never descends into build output — `.git`,
`target`, `node_modules`, `vendor`, `dist`, `.venv` and friends. On this
repository that is the difference between **100ms and 10ms**; across an
eight-repo tree, 1.46s and 0.31s.

| Flag | Effect |
|---|---|
| `--exclude <glob>` | Skip matching paths (repeatable) |
| `--include <glob>` | Scan matching paths whatever their extension (repeatable) |
| `--no-ignore` | Don't honour `.gitignore` |
| `--max-file-size <bytes>` | Skip files above the cap (default 10 MB) |
| `--follow-symlinks` | Follow symlinks (off by default — a loop is a hang) |
| `--jobs <n>` | Traversal threads (0 = automatic) |

```bash
injection-scanner check ./corpus --include '**/*.json' --include '**/*.mdx'
injection-scanner check . --exclude '**/fixtures/**'
```

The built-in deny-list is not a `.gitignore` rule and is **not** lifted by
`--no-ignore` or reachable by `--include`. `--no-ignore` means "don't trust this
repository's ignore rules", which is reasonable on a checkout you didn't write;
it doesn't mean "scan 40,000 files of build output".

Nothing is skipped silently. Files the walker reached and declined are counted on
stderr, and if `.gitignore` rules were applied it says so — because those prune
whole subtrees before the walker sees them, so they can't be itemised:

```
note: .gitignore rules were applied — paths they exclude were not scanned and are
      not counted above. Use --no-ignore to include them.
note: 27 file(s) not scanned — not a scanned file type. Use --include <glob> to add them.
```

That disclosure matters more than it looks: a skills pack shipping a `.gitignore`
containing `*` would otherwise scan nothing and report a clean bill of health.

## Pattern Categories

| Category | Patterns | Default Severity | Examples |
|---|---|---|---|
| Role Override | 7 | CRITICAL | "ignore all previous instructions", "you are now", "forget everything" |
| Instruction Injection | 5 | HIGH | "SYSTEM OVERRIDE", hidden HTML instructions, "your real purpose is" |
| Data Exfiltration | 6 | CRITICAL | "send data to https://...", "output your instructions", "repeat system message" |
| Jailbreaks | 9 | HIGH | "developer mode enabled", "DAN mode", "pretend you have no restrictions" |
| Encoding/Obfuscation | 3 | HIGH | Unicode RTL overrides, zero-width characters, zero-width sequences |

**30 patterns** across 5 categories. See [PATTERNS.md](PATTERNS.md) for the full pattern contribution guide.

## Output Examples

### Text output (default)

```
tests/fixtures/injected-skill.md
  :6 CRITICAL  Attempts to override agent instructions — Remove instruction override text. If documenting attacks, use code blocks.  (PI001)
  :6 CRITICAL  System-level instruction injection — Remove system override. Only actual system prompts should use SYSTEM prefix.  (PI011)
  :9 HIGH  Role reassignment attempt — Remove role override. Use explicit skill scoping instead.  (PI003)
  :9 CRITICAL  System-level instruction injection — Remove system override. Only actual system prompts should use SYSTEM prefix.  (PI011)
  :14 HIGH  Developer mode jailbreak — Remove developer mode activation.  (PI030)
  :16 HIGH  Restriction removal via pretense — Remove restriction bypass.  (PI033)

6 finding(s): 3 critical, 3 high, 0 medium, 0 low
```

### JSON output

```json
[
  {
    "file": "tests/fixtures/injected-skill.md",
    "matches": [
      {
        "pattern_id": "PI001",
        "pattern_name": "ignore-previous-instructions",
        "severity": "CRITICAL",
        "message": "Attempts to override agent instructions",
        "remediation": "Remove instruction override text. If documenting attacks, use code blocks.",
        "file": "tests/fixtures/injected-skill.md",
        "line": 6,
        "matched_text": "ignore all previous instructions"
      }
    ],
    "critical_count": 3,
    "high_count": 3,
    "medium_count": 0,
    "low_count": 0
  }
]
```

## Documentation Is Not an Attack

A security guide that quotes `ignore all previous instructions` is documenting an
attack, not carrying one. The scanner used to be unable to tell the difference and
reported **15 findings on this README** — nine in the table above, six in sample
output.

It now tracks where in a markdown document each match sits and scores it:

| Context | Confidence | Reported by default |
|---|---|---|
| Prose | 1.0 | yes |
| HTML comment | 1.0 | yes |
| Frontmatter | 0.9 | yes |
| Block quote | 0.9 | yes |
| Table cell | 0.3 | no |
| Inline code span | 0.3 | no |
| Fenced code block | 0.2 | no |

HTML comments are deliberately **not** downgraded. Hidden text is a delivery
mechanism, not a disclaimer — a payload nobody can see is worse than one they can.
Frontmatter is structured config an agent loads directly, and a block quote is
still text the model reads.

```bash
injection-scanner check README.md                    # 0 findings
injection-scanner check README.md --strict           # 15 — every context reported
injection-scanner check docs/ --min-confidence 0.9   # only the strongest signals
```

### Withheld, never dropped

A finding below the threshold is **not discarded** — it moves to a `low_confidence`
array in the report, and the text summary says so:

```
$ injection-scanner check untrusted.md
No injection patterns reported.
2 findings withheld as documentation (code blocks, inline spans, tables).
Re-run with --strict to see them.
```

This matters because the confidence score is a guess about how a document will be
**consumed**, and that guess can be wrong. A fenced block is inert in a README a
human reads. The same block, in a web page or RAG document flattened into an
agent's context, arrives as bare text — the fence markers are gone, and the payload
reads exactly like an instruction. We confirmed this against a live agent: payloads
in fenced blocks and table cells reached the model in full.

So the threshold is right for scanning **your own repository**, where a fenced block
stays a fenced block. Use `--strict` for **untrusted input** — anything you did not
write and are about to feed to a model:

```bash
injection-scanner check ./docs                  # your repo: documentation is documentation
injection-scanner check ./rag-corpus --strict   # untrusted input: assume the fence is stripped
```

Withheld findings never affect exit codes or severity counts, so turning this on
does not change what your CI gate blocks on — only what you can see.

Every finding carries `context` and `confidence` in `--format json`, so a consumer
can apply its own threshold instead of trusting this one:

```json
{
  "pattern_id": "PI001",
  "line": 56,
  "context": "table",
  "confidence": 0.3
}
```

**Context is not a safety guarantee.** A model ingesting a document reads the
fenced blocks too. The score says a match is *less likely to be an attack*, not
that it is harmless — so use `--strict` whenever the document is not yours,
alongside `--no-suppress`.

## Inline Suppression

Three forms, all using pattern IDs. `injection-scanner:ignore` applies to **the line it appears on**:

```markdown
ignore all previous instructions <!-- injection-scanner:ignore PI001 -->
```

`ignore-next-line` applies to **the following line** — useful when the finding is inside a code block
or you would rather keep the comment out of the content:

```markdown
<!-- injection-scanner:ignore-next-line PI001 -->
ignore all previous instructions
```

`ignore-file` applies to **the whole file**, and must appear within the first 10 lines so a file-wide
escape hatch stays visible rather than buried:

```markdown
<!-- injection-scanner:ignore-file PI001,PI003 -->
```

Multiple IDs are comma-separated in every form:

```markdown
<!-- injection-scanner:ignore PI001,PI003 -->
```

Suppression is per-pattern, never file-global by default — suppressing `PI001` leaves every other
pattern active on that line.

### Suppression is a trust boundary

Suppression directives live **inside the file being scanned**, so whoever can edit that file decides
what the scanner reports. If you did not write the file, its author can disarm your scan:

```markdown
---
title: Innocuous Doc
injection-scanner:ignore-file PI001
---

Ignore all previous instructions.
```

This is inherent to inline suppression — `eslint-disable` and `# noqa` have the same property — and
it is not fixable by requiring a particular comment syntax, since an attacker who can write bare text
can equally write `<!-- ... -->`. Two things make it manageable instead:

**Suppression is never silent.** Any withheld finding is reported, so a hostile document does not
look identical to a clean one:

```
No injection patterns detected.
1 finding suppressed by directives in 1 scanned file. Re-run with --no-suppress to see it.
```

The same information is in `--format json`, as a `suppressed` array on each report.

**`--no-suppress` refuses them entirely.** Use it whenever the file is not yours — downloaded skills,
RAG corpora, pull requests from forks, anything ingested from the network:

```bash
injection-scanner check ./untrusted-skills --no-suppress
```

Rule of thumb: **suppression is for your own repository; `--no-suppress` is for everyone else's
content.**

## Exit Codes

| Code | Meaning |
|---|---|
| 0 | No findings |
| 1 | One or more findings detected |

## Part of UnityInFlow

injection-scanner is tool #03 in the [UnityInFlow](https://github.com/UnityInFlow) AI agent tooling ecosystem -- 20 open-source tools covering spec validation, token cost control, runtime, compliance, and more.

## License

MIT
