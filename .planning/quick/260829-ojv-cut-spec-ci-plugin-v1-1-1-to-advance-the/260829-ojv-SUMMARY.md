---
id: 260829-ojv
slug: cut-spec-ci-plugin-v1-1-1-to-advance-the
date: 2026-08-29
mode: quick
status: complete
---

# Summary — v1.1.1 published, `v1` moved, the chain is closed

The v0.1.0 detection work now reaches `@v1` consumers. This was the last inert link.

## What shipped

| Step | Result |
|---|---|
| `package.json` 1.1.0 → 1.1.1 | PR #14, rebase-merged → `d3069c4`, 0 merge commits |
| Tag `v1.1.1` + Release published | https://github.com/UnityInFlow/spec-ci-plugin/releases/tag/v1.1.1 |
| Major-tag mover | Run 33261004298, **success in 9s** |
| `v1` | `5adc903` → **`d3069c4`**; main is now 0 commits ahead |

## Verified at the `v1` tree — what a consumer actually gets

Not inferred from "the workflow said success". Read out of the tag itself:

- `src/injection-scanner.ts:34` → `DEFAULT_SCANNER_VERSION = "v0.1.0"`
- `action.yml:25` → `default: "v0.1.0"`
- `README.md:78` defaults row → `v0.1.0`
- **`dist/index.js`** — the bundle consumers execute, not just the source — contains `"v0.1.0"`
- `git cat-file -t v1` → `tag`, and `gh release view v1` → **`release not found`**

That last check matters: `release.yml` states in capitals that `v1` must never become a Release
object, because it would be incompatible with immutable releases. It did not become one.

## Why patch and not minor

No input, output, or behaviour of the Action changed — only the default value of an existing
input. The scanner's detection improved; this Action's contract did not.

`dist/index.js` was byte-identical after rebuild (the version is not embedded in the bundle), so
the diff is a single line in `package.json` and the dist-diff gate stays satisfied.

## The mover worked, and it was right not to assume so

The three commits preceding v1.1.0 are all fixes to this exact workflow, and at v1.0.0 it queued
on an offline runner fleet and `v1` had to be moved by hand. The plan therefore treated
"published the release" and "`v1` moved" as two separate facts and checked the second one
independently. It had in fact run — but the audited `workflow_dispatch` fallback with a `tag`
input was the ready answer if it had not, rather than a hand force-push.

## The shape of the bug this closed, worth carrying forward

**A moving alias tag makes "merged" and "released" and "reaching users" three different states,
and only the third one counts.** injection-scanner v0.1.0 was published and correct. The
consumer's default was merged and correct. Every gate was green. And zero users had the fix,
because a plain git tag one commit behind sat silently between them.

Nothing in either repo's CI could have caught that — the gap is *between* repos, in a tag whose
whole job is to be stale until someone moves it. Any future scanner release has to end with the
same three-step chain, and the third step is the one that will be forgotten.
