# Pattern Contribution Guide

injection-scanner uses a YAML-based pattern library. Core patterns are embedded at compile time. Community patterns can be added via PR.

## Pattern Format

```yaml
category: category_name
default_severity: CRITICAL  # CRITICAL | HIGH | MEDIUM | LOW
patterns:
  - id: PI0XX
    name: descriptive-name
    pattern: "regex\\s+pattern"
    severity: HIGH  # optional -- overrides category default
    description: "What this detects"
    remediation: "How to fix"
    tags: [tag1, tag2]
```

## Categories

| Category | ID Range | Default Severity |
|---|---|---|
| Role Override | PI001-PI009 | CRITICAL |
| Instruction Injection | PI010-PI019 | HIGH |
| Data Exfiltration | PI020-PI029 | CRITICAL |
| Jailbreaks | PI030-PI039 | HIGH |
| Encoding/Obfuscation | PI040-PI049 | HIGH |

## Submitting a Pattern

1. Fork the repo
2. Add pattern to the appropriate `patterns/core/*.yaml` file
3. Include in your PR:
   - At least 3 true positive test cases
   - At least 2 non-match cases (false positive prevention)
4. Run `cargo test`
5. Submit PR with title: `feat: add PI0XX pattern-name`
