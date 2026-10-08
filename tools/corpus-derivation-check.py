#!/usr/bin/env python3
"""GATE-01 derivation check: prove corpus payloads were not adapted from barred text.

GATE-01 says attack-corpus payloads are written from the threat model and are
never derived from the patterns (or from material tuned against them). The
only enforcement that does not depend on the author's honesty is ordering, and
ordering cannot see a payload that was *paraphrased* from a barred source --
a lifted sentence with two words changed is still a lifted sentence.

This script is the mechanical half. Given one or more BARRED texts and a set
of payload files, it fails when any payload sentence

  * shares a 5-word n-gram with a barred sentence, or
  * has a token-set Jaccard similarity above 0.6 with a barred sentence.

Both thresholds are calibrated and fixed (see the constants below); they are
deliberately not command-line flags. Measured against Phase 5's own barred
document: five blind-written payloads scored 0 collisions, sentences lifted
verbatim scored 5 to 14, and a lift with two words changed still scored 9
n-grams at Jaccard 0.94. So a 5-gram hit is a copying signal rather than a
generic-phrasing accident. If it fires, rewrite the payload; do not loosen the
gate.

The script reads the barred files itself so that the agent authoring payloads
never has to see them. For the same reason the report names the offending
PAYLOAD n-gram and the barred file and line number, and never prints barred
text.

Usage:
    corpus-derivation-check.py --barred SRC [--barred SRC ...] --payloads FILE [FILE ...]

    SRC is a file path, or `<git-rev>:<path>` to read a blob from history (use
    a recorded sha, not a relative ref, so the check survives later commits).

Exit status: 0 no collision, 1 at least one collision, 2 usage or read error.

Payload files come in three shapes, detected from their content:
  * a whole-file JSON document -- only its string VALUES are payload text; the
    keys are schema chosen by the host, not by the author, and including them
    would flag every hooks file for sharing the word `command`;
  * a `---`/`+++` frontmatter document -- every line is payload text;
  * a line-oriented corpus file -- every non-blank line not starting with `#`.
"""

from __future__ import annotations

import json
import re
import subprocess
import sys
from pathlib import Path

NGRAM = 5
JACCARD_LIMIT = 0.6
# A sentence shorter than this is too short for set similarity to mean anything
# ("Done." against "Done." is Jaccard 1.0). The 5-gram test still applies to it.
MIN_JACCARD_TOKENS = 6

TOKEN = re.compile(r"[a-z0-9]+")
SENTENCE_SPLIT = re.compile(r"(?<=[.!?])\s+|\n+")


def tokens(text: str) -> list[str]:
    """Lowercase, drop punctuation, split into alphanumeric words."""
    return TOKEN.findall(text.lower())


def sentences(text: str) -> list[str]:
    return [s for s in SENTENCE_SPLIT.split(text) if s.strip()]


def ngrams(words: list[str]) -> set[tuple[str, ...]]:
    return {tuple(words[i : i + NGRAM]) for i in range(len(words) - NGRAM + 1)}


def read_barred(spec: str) -> tuple[str, str]:
    """Return (label, text) for a path or a `<rev>:<path>` git blob."""
    path = Path(spec)
    if path.is_file():
        return spec, path.read_text(encoding="utf-8")
    if ":" in spec:
        result = subprocess.run(
            ["git", "show", spec], capture_output=True, text=True, check=False
        )
        if result.returncode == 0:
            return spec, result.stdout
        sys.stderr.write(f"error: `git show {spec}` failed: {result.stderr.strip()}\n")
        sys.exit(2)
    sys.stderr.write(f"error: barred source {spec!r} is neither a file nor rev:path\n")
    sys.exit(2)


def json_string_values(node: object) -> list[str]:
    """Every string value in a parsed JSON document, keys excluded."""
    if isinstance(node, str):
        return [node]
    if isinstance(node, list):
        return [v for item in node for v in json_string_values(item)]
    if isinstance(node, dict):
        return [v for item in node.values() for v in json_string_values(item)]
    return []


def payload_texts(path: Path) -> list[str]:
    """The authored text of one payload file, one entry per sentence-ish unit."""
    raw = path.read_text(encoding="utf-8")
    stripped = raw.lstrip()
    if stripped.startswith("{"):
        try:
            return json_string_values(json.loads(raw))
        except json.JSONDecodeError:
            pass  # fall through: a malformed JSON payload is still text
    if stripped.startswith("---") or stripped.startswith("+++"):
        return [line for line in raw.splitlines() if line.strip()]
    return [
        line
        for line in raw.splitlines()
        if line.strip() and not line.lstrip().startswith("#")
    ]


class Barred:
    """Index of every barred sentence: n-gram -> location, plus token sets."""

    def __init__(self) -> None:
        self.gram_at: dict[tuple[str, ...], tuple[str, int]] = {}
        self.sets: list[tuple[frozenset[str], str, int]] = []

    def add(self, label: str, text: str) -> None:
        for line_no, line in enumerate(text.splitlines(), start=1):
            # n-grams run over the whole line (stricter than per sentence: a run
            # that straddles a sentence boundary is still copied text); the
            # similarity test runs per sentence.
            for gram in ngrams(tokens(line)):
                self.gram_at.setdefault(gram, (label, line_no))
            for sentence in sentences(line):
                words = tokens(sentence)
                if len(words) >= MIN_JACCARD_TOKENS:
                    self.sets.append((frozenset(words), label, line_no))


def check_payload(path: Path, barred: Barred) -> list[str]:
    findings: list[str] = []
    for text in payload_texts(path):
        for gram in sorted(ngrams(tokens(text))):
            hit = barred.gram_at.get(gram)
            if hit is not None:
                findings.append(
                    f"{path}: shares the {NGRAM}-word run {' '.join(gram)!r} "
                    f"with {hit[0]} line {hit[1]}"
                )
        for sentence in sentences(text):
            words = tokens(sentence)
            if len(words) >= MIN_JACCARD_TOKENS:
                mine = frozenset(words)
                for theirs, label, line_no in barred.sets:
                    union = len(mine | theirs)
                    similarity = len(mine & theirs) / union if union else 0.0
                    if similarity > JACCARD_LIMIT:
                        findings.append(
                            f"{path}: token-Jaccard {similarity:.2f} (> {JACCARD_LIMIT}) "
                            f"against {label} line {line_no}; sentence starts "
                            f"{sentence[:60]!r}"
                        )
                        break
    return findings


def parse_args(argv: list[str]) -> tuple[list[str], list[Path]]:
    barred: list[str] = []
    payloads: list[Path] = []
    mode = ""
    for arg in argv:
        if arg == "--barred":
            mode = "barred"
        elif arg == "--payloads":
            mode = "payloads"
        elif mode == "barred":
            barred.append(arg)
            mode = ""
        elif mode == "payloads":
            payloads.append(Path(arg))
        else:
            sys.stderr.write(f"error: unexpected argument {arg!r}\n")
            sys.exit(2)
    if not barred or not payloads:
        sys.stderr.write(
            "usage: corpus-derivation-check.py --barred SRC [--barred SRC ...] "
            "--payloads FILE [FILE ...]\n"
        )
        sys.exit(2)
    return barred, payloads


def main(argv: list[str]) -> int:
    barred_specs, payload_paths = parse_args(argv)

    index = Barred()
    for spec in barred_specs:
        label, text = read_barred(spec)
        index.add(label, text)

    findings: list[str] = []
    for path in payload_paths:
        if not path.is_file():
            sys.stderr.write(f"error: payload {path} is not a file\n")
            return 2
        findings.extend(check_payload(path, index))

    for finding in findings:
        print(finding)
    if findings:
        print(
            f"FAIL: {len(findings)} collision(s) -- a payload is derived from barred "
            "text. Rewrite it; the thresholds are calibrated and are not to be loosened."
        )
        return 1
    print(
        f"OK: {len(payload_paths)} payload file(s) share no {NGRAM}-word run and no "
        f"> {JACCARD_LIMIT} token-Jaccard sentence with {len(barred_specs)} barred source(s)."
    )
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
