#!/usr/bin/env bash
# Scanner side: what injection-scanner reports, default vs --strict, per document.
set -euo pipefail
LAB="$(cd "$(dirname "$0")" && pwd)"
# `$LAB` is `tools/injection-lab`, so `../..` is the repository root — the
# default resolves on any checkout. It previously hardcoded one developer's
# absolute home path (#123), which worked on exactly one machine and published
# that machine's layout in a public repository.
BIN="${INJECTION_SCANNER:-$LAB/../../target/release/injection-scanner}"
[ -x "$BIN" ] || { echo "set INJECTION_SCANNER to the binary path"; exit 1; }

printf '%-30s %-14s %8s %8s\n' DOCUMENT CONTEXT DEFAULT STRICT
printf '%-30s %-14s %8s %8s\n' "------------------------------" "--------------" "-------" "------"

python3 - "$BIN" "$LAB" <<'PY'
import json, subprocess, sys, pathlib
binary, lab = sys.argv[1], pathlib.Path(sys.argv[2])
manifest = json.loads((lab / "manifest.json").read_text())

def count(path, strict):
    cmd = [binary, "check", str(path), "--format", "json"] + (["--strict"] if strict else [])
    out = subprocess.run(cmd, capture_output=True, text=True).stdout
    try:
        return len(json.loads(out)[0]["matches"])
    except Exception:
        return 0

for name, meta in manifest.items():
    f = lab / "corpus" / f"{name}.md"
    d, s = count(f, False), count(f, True)
    flag = ""
    if meta["canary"] and d == 0:
        flag = "  <- payload present, NOT reported by default"
    print(f"{name:<30} {meta['context']:<14} {d:>8} {s:>8}{flag}")
PY
