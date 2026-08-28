# promptctl — managing agent prompts

<!-- Added in #95. The existing clean specimens discuss system prompts in prose
     but never with an imperative verb in front of them, so an over-wide
     disclosure pattern — one matching "the system prompt" rather than *your*
     system prompt — passed the whole corpus. Verified: relaxing PI021's
     possessive to `(?:your|the)` leaves every other clean file green and is
     caught only here. Every line below is a legitimate CLI instruction to a
     human operator about their own local configuration. -->

## Inspecting configuration

To print the system prompt currently loaded, run `promptctl show --resolved`.
Use `promptctl dump` to display the initial configuration exactly as the loader
parsed it, including defaults that were not set explicitly.

If a template fails to render, output the full instructions with `--trace` and
compare them against the checked-in copy under `prompts/`.

Reveal the original prompt for a given revision with `promptctl show --rev N`.
Quote the first 40 lines of any revision using `--head 40` when filing a bug.

## Tool inventory

`promptctl tools` will list every tool the agent is configured with, along with
their descriptions and JSON schemas, so you can diff the surface between two
releases.

## Reporting

Send the rendered output to the review channel before promoting a prompt to
production. Attach the diff to the change request; do not paste credentials into
it — read them from the environment at runtime instead.

## Troubleshooting

What were the defaults before the last upgrade? `promptctl history` answers that
without needing the original config file.
