#!/usr/bin/env bash
# Run with: bash tests/gate03_sweep_test.sh
set -euo pipefail

repo_dir="$(cd "$(dirname "$0")/.." && pwd)"
# Load the real helpers without executing the command-line entry point.
source <(sed '/^main "\$@"$/d' "${1:-$repo_dir/scripts/gate03-sweep.sh}")

scratch="$(mktemp -d)"
trap 'rm -rf -- "$scratch"' EXIT
mkdir -p "$scratch/input" "$scratch/output" "$scratch/empty"

# A deterministic scanner substitute exercises the helpers' JSON processing.
fake_scanner() {
  printf '[{"file":"probe.md","matches":[{"line":1,"pattern_id":"PI001","severity":"CRITICAL"}]}]\n'
  return "$scanner_status"
}
BIN=fake_scanner
require_binary() { :; }

out_dir=outer_out_dir dir=outer_dir abs_dir=outer_abs_dir slug=outer_slug
report=outer_report status=outer_status files_scanned=outer_files findings=outer_findings
before="$(declare -p out_dir dir abs_dir slug report status files_scanned findings)"

scanner_status=0
sweep_one "$scratch/output" "$scratch/input"
build_summary "$scratch/output"
[[ "$(declare -p out_dir dir abs_dir slug report status files_scanned findings)" == "$before" ]]

for scanner_status in 0 1 2; do
  main "$scratch/output" "$scratch/input" "$scratch/missing"
  [[ "$(declare -p out_dir dir abs_dir slug report status files_scanned findings)" == "$before" ]]
  [[ "$(wc -l < "$scratch/output/manifest.tsv")" -eq 2 ]]
  grep -q $'\t1\t1\tswept$' "$scratch/output/manifest.tsv"
  grep -q $'\t0\t0\tskipped-missing$' "$scratch/output/manifest.tsv"
  [[ "$(cat "$scratch/output/summary.tsv")" == $'PI001\tCRITICAL\t1' ]]
done

do_compare "$scratch/output" "$scratch/output"
if do_compare "$scratch/empty" "$scratch/output" > "$scratch/delta"; then
  echo 'comparison must return 1 for a new finding' >&2
  exit 1
else
  [[ "$?" -eq 1 ]]
fi
[[ "$(cat "$scratch/delta")" == $'probe.md:1\tPI001' ]]
[[ "$(declare -p out_dir dir abs_dir slug report status files_scanned findings)" == "$before" ]]
echo 'gate03 sweep helper checks passed'
