# Documentation corpus

Documents that **quote real payloads** and must still report zero findings at
the default threshold — because the payloads sit in fenced blocks, inline spans
or table cells, which is what writing about injection looks like.

The distinction from `../clean/` is deliberate and enforced in both directions:

| | default | `--strict` |
|---|---|---|
| `clean/` | 0 findings | **0 findings** — nothing matches at all |
| `documentation/` | 0 findings | **> 0 findings** — context awareness is doing the work |

That second row is the point. A file here that is also clean under `--strict`
has stopped testing anything, and the suite says so. Without it, someone could
"fix" a false positive by widening the context downgrade until it swallowed real
attacks, and every test would still pass.
