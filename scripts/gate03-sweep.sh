#!/usr/bin/env bash
#
# gate03-sweep.sh -- GATE-03's third-party sweep, reconstructed as a runnable
# script.
#
# GATE-03 is the gate that caught the ENG-02 production panic that sixteen
# green unit tests missed: a fixed-byte-offset slice that only breaks on real
# multi-byte bytes in a real third-party file. The historical sweep that
# caught it (PR #103/#102, roughly 1,300 files) was a manual, machine-local
# run against `~/.claude/plugins/cache`, a GSD workflow reference set and
# seven sibling repositories. It was never a script, never vendored into this
# repository, and never a CI job.
#
# This script does not pretend that corpus is reproducible. There is no fixed
# directory list this repository can commit to and expect to exist on every
# machine, so every run enumerates whatever directories are actually present
# and records exactly what it found -- file counts, finding counts, and which
# candidates were skipped because they were not there. An absolute "N
# findings" number from one run is not evidence of anything; the meaningful
# signal is the delta `--compare` computes between a pre-change and a
# post-change run over the SAME directory list, on the SAME machine.
#
# Usage:
#   gate03-sweep.sh <output-dir> <dir> [dir...]
#       Runs the release scanner over each <dir>, writing one JSON report per
#       directory into <output-dir>, plus manifest.tsv (per-directory file and
#       finding counts) and summary.tsv (findings aggregated by pattern id and
#       severity).
#
#   gate03-sweep.sh --compare <baseline-output-dir> <candidate-output-dir>
#       Diffs the findings recorded in two prior sweep runs, keyed on file
#       path, line number and pattern id. Prints every finding present in the
#       candidate and absent from the baseline. Exits 1 if that set is
#       non-empty, 0 otherwise.
#
# Environment:
#   INJECTION_SCANNER_BIN   Path to the scanner binary. Defaults to
#                           target/release/injection-scanner. The sweep must
#                           run the RELEASE binary against real directories --
#                           substituting `cargo test` here is a recorded
#                           blocking anti-pattern for this milestone, because
#                           unit tests over synthetic fixtures are exactly
#                           what missed the ENG-02 panic.
set -euo pipefail

BIN="${INJECTION_SCANNER_BIN:-target/release/injection-scanner}"

usage() {
  cat >&2 <<'USAGE'
Usage:
  gate03-sweep.sh <output-dir> <dir> [dir...]
  gate03-sweep.sh --compare <baseline-output-dir> <candidate-output-dir>
USAGE
}

# Turn an absolute path into a filesystem-safe, unique file name for the
# per-directory JSON report.
slugify() {
  printf '%s' "$1" | sed -e 's#^/##' -e 's#[^A-Za-z0-9._-]#_#g'
}

require_binary() {
  if [ ! -x "$BIN" ]; then
    echo "ERROR: scanner binary not found or not executable at '$BIN'." >&2
    echo "Build it with: cargo build --release" >&2
    echo "Or set INJECTION_SCANNER_BIN to point at an existing binary." >&2
    exit 1
  fi
}

# sweep_one <output-dir> <dir>
#
# A directory that is not present on this machine is not an error -- the
# corpus is machine-local by construction -- so it is recorded as a skip in
# the manifest rather than aborting the whole run.
sweep_one() {
  out_dir="$1"
  dir="$2"

  if [ ! -d "$dir" ]; then
    echo "WARNING: skipping '$dir' -- not present on this machine" >&2
    printf '%s\t%s\t%s\t%s\n' "$dir" "0" "0" "skipped-missing" >>"${out_dir}/manifest.tsv"
    return 0
  fi

  abs_dir="$(cd "$dir" && pwd -P)"
  slug="$(slugify "$abs_dir")"
  report="${out_dir}/${slug}.json"

  # The scanner's own exit codes distinguish "clean" (0), "found something at
  # or above --fail-on" (1) and "found something below --fail-on" (2). All
  # three are a completed run -- findings are the measurement this sweep
  # wants, not a failure of the sweep itself. Only a signal or a missing
  # binary is an actual error.
  set +e
  "$BIN" check "$abs_dir" \
    --format json \
    --all-files \
    --no-ignore \
    --exclude '.planning/**' \
    >"$report"
  status=$?
  set -e

  case "$status" in
    0 | 1 | 2) : ;;
    *)
      echo "ERROR: scanner exited $status (not 0/1/2) while scanning '$abs_dir' -- treating as a real failure, not a finding." >&2
      exit 1
      ;;
  esac

  files_scanned=$(python3 -c "
import json, sys
print(len(json.load(open(sys.argv[1]))))
" "$report")

  findings=$(python3 -c "
import json, sys
data = json.load(open(sys.argv[1]))
print(sum(len(r['matches']) for r in data))
" "$report")

  printf '%s\t%s\t%s\t%s\n' "$abs_dir" "$files_scanned" "$findings" "swept" >>"${out_dir}/manifest.tsv"
}

# build_summary <output-dir>
#
# Aggregates findings across every per-directory JSON report already written
# into <output-dir> by pattern id and severity.
build_summary() {
  out_dir="$1"
  python3 -c "
import glob, json, os, sys
from collections import Counter

out_dir = sys.argv[1]
counts = Counter()
for path in glob.glob(os.path.join(out_dir, '*.json')):
    with open(path) as handle:
        data = json.load(handle)
    for report in data:
        for match in report['matches']:
            counts[(match['pattern_id'], match['severity'])] += 1

with open(os.path.join(out_dir, 'summary.tsv'), 'w') as handle:
    for (pattern_id, severity), count in sorted(counts.items()):
        handle.write(f'{pattern_id}\t{severity}\t{count}\n')
" "$out_dir"
}

# do_compare <baseline-output-dir> <candidate-output-dir>
#
# The actual GATE-03 evidence: what changed between two runs over the same
# directory list, keyed on (file, line, pattern id) so a finding that merely
# moved line number because of an unrelated edit is not confused with a new
# one at the wrong line.
do_compare() {
  python3 -c "
import glob, json, os, sys

def load_findings(out_dir):
    findings = set()
    for path in glob.glob(os.path.join(out_dir, '*.json')):
        with open(path) as handle:
            data = json.load(handle)
        for report in data:
            for match in report['matches']:
                findings.add((report['file'], match['line'], match['pattern_id']))
    return findings

baseline = load_findings(sys.argv[1])
candidate = load_findings(sys.argv[2])
new = sorted(candidate - baseline)

for file, line, pattern_id in new:
    print(f'{file}:{line}\t{pattern_id}')

sys.exit(1 if new else 0)
" "$1" "$2"
}

main() {
  if [ "$#" -eq 0 ]; then
    usage
    exit 2
  fi

  if [ "$1" = "--compare" ]; then
    if [ "$#" -ne 3 ]; then
      usage
      exit 2
    fi
    do_compare "$2" "$3"
    return $?
  fi

  out_dir="$1"
  shift

  if [ "$#" -eq 0 ]; then
    usage
    exit 2
  fi

  require_binary
  mkdir -p "$out_dir"
  rm -f "${out_dir}"/*.json "${out_dir}/manifest.tsv" "${out_dir}/summary.tsv" 2>/dev/null || true
  : >"${out_dir}/manifest.tsv"

  for dir in "$@"; do
    sweep_one "$out_dir" "$dir"
  done

  build_summary "$out_dir"
}

main "$@"
