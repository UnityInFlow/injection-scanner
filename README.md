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

## Inline Suppression

Suppress specific patterns on a line by adding a comment on the line above:

```markdown
<!-- injection-scanner:ignore PI001 -->
ignore all previous instructions
```

Multiple patterns can be suppressed:

```markdown
<!-- injection-scanner:ignore PI001,PI003 -->
```

## Exit Codes

| Code | Meaning |
|---|---|
| 0 | No findings |
| 1 | One or more findings detected |

## Part of UnityInFlow

injection-scanner is tool #03 in the [UnityInFlow](https://github.com/UnityInFlow) AI agent tooling ecosystem -- 20 open-source tools covering spec validation, token cost control, runtime, compliance, and more.

## License

MIT
