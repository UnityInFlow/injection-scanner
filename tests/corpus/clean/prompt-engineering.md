# Prompt engineering notes

<!-- Would-be PI011. `(?i)\[?SYSTEM\s*(OVERRIDE|PROMPT|MESSAGE)\]?` made the
     brackets optional, so the ordinary technical term matched at CRITICAL. Every
     document in this ecosystem that discusses prompts fired it. -->

The system prompt is loaded from config at startup and cached for the process
lifetime. Changing it requires a restart.

We log the system message id for tracing, but never its contents.

When the system prompt exceeds the context budget, the loader truncates the
examples section first and warns.
