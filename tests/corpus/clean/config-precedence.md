# Configuration precedence

<!-- Added in #97. The precedence arm of PI018 targets a payload claiming authority
     over the *user* or the *conversation*. "X takes precedence over Y" is also the
     single most common sentence in configuration documentation, and nothing in the
     corpus said so — so the first draft of that arm matched every line below.
     Verified: broadening the arm's object list back to a bare `user|system` turns
     this file red and leaves every other clean specimen green. -->

Settings are merged from three locations. Project settings take precedence over
user settings, and both take precedence over the packaged defaults.

A value set on the command line has higher priority than any file, including the
project file. Environment variables sit between the two: they supersede the
packaged defaults but not an explicit flag.

## Rule ordering

Where two lint rules disagree, the more specific one takes precedence over the
general one. The rules in this document take precedence over the older wiki page,
which is retained only for historical reference.

## Overrides

A per-directory override supersedes the repository root configuration for files
beneath it. This is the only case where a nested file overrides the user's global
preference, and it is deliberate.

Note that `--strict` overrides the configured confidence threshold; it does not
merge with it.
