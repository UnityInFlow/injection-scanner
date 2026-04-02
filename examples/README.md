# Example Attack Files

These files demonstrate prompt injection patterns that `injection-scanner` detects.

## Usage

```bash
# Scan all examples
injection-scanner check examples/

# Scan a specific category
injection-scanner check examples/role-override-attack.md

# See JSON output
injection-scanner check examples/ --format json
```

## Files

| File | Category | Patterns | Expected Findings |
|---|---|---|---|
| `clean-skill.md` | None | — | 0 (clean) |
| `role-override-attack.md` | A: Role Override | PI001-PI007 | 5 CRITICAL |
| `instruction-injection-attack.md` | B: Instruction Injection | PI010-PI014 | 4 HIGH |
| `exfiltration-attack.md` | C: Data Exfiltration | PI020-PI025 | 4 CRITICAL |
| `jailbreak-attack.md` | D: Jailbreaks | PI030-PI038 | 5 HIGH |
| `mixed-attack.md` | All categories | Multiple | 7+ findings |
| `suppressed-documentation.md` | Inline suppression | Suppressed | 1 finding (unsuppressed) |
