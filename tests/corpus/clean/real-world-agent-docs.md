# Agent tooling notes

<!-- Added in #102. Every line below is copied, near-verbatim, from a REAL
     document that the release binary reported at HIGH or CRITICAL: third-party
     plugin skills under ~/.claude/plugins, the GSD workflow reference set, and
     sibling repositories in this ecosystem. ~1000 files swept; these are the
     shapes that produced false positives.

     They are here because the previous specimens were all written by someone
     who already knew which pattern they were testing. These were not. -->

## Tool safety

**NEVER use the Write tool on an existing file in fix mode.** Write replaces the
entire file with whatever you provide — any content not in your context window is
permanently destroyed. Edit makes targeted replacements and is the only safe tool
for fix mode.

Avoid the convenience installer: it silently executes unverified packages from
the registry. Download, read, then run.

## Log viewer

**Expected:** logs appear with a `[server-name]` prefix. Clicking the
"echo-server" pill navigates to the filtered view and only echo-server lines
appear. Clicking "All" returns every server's output.

## Planning references

**Important: Skip deferred items.** When reading the verification report, only
the gaps section contains actionable items. The deferred section lists work
already scheduled for later phases.

Record the migration in the table — then continue as above. Do this migration
once; do not repeat it once the table exists.

## Rendered HTML in reference files

Some reference pages are captured from upstream documentation and keep their
original markup, including accessibility attributes:

<div aria-hidden="true" class="QueryBuilder-Sizer"></div>
<div class="Overlay--hidden" data-modal-dialog-overlay></div>

Those hide nothing from a reader; `aria-hidden` is an accessibility hint.

## Specification links

[JCache](https://jcp.org/en/jsr/detail?id=107) is bootstrapped through the
provider, and the servlet spec is [JSR 369](https://jcp.org/en/jsr/detail?id=369).
A query string carrying an identifier is not a beacon.
