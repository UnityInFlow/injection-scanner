# CLAUDE.md — payments service

<!-- The single most common document this scanner is pointed at. It is full of
     imperatives addressed to a model, which is exactly the shape an injection
     has; the difference is provenance, not phrasing. If this file cannot stay
     clean, the tool cannot be run on its primary target. -->

## Project Overview

Kotlin service handling card authorisation and settlement.

## Constraints

- You must run `./gradlew ktlintFormat` before every commit
- Never commit secrets — all credentials come from the environment
- Always use `Result<T>` for expected failures; do not throw
- Ignore the legacy `v1/` package; it is scheduled for deletion
- Forget the old naming convention — everything is `io.github.acme` now

## Your role

You are the maintainer of this service. Act as a careful reviewer: read the
surrounding code before changing it, and prefer the smallest change that works.

When responding to a failing test, first reproduce it locally, then explain what
you found before proposing a fix.

## Acceptance Criteria

- [ ] Authorisation completes in under 300ms at p99
- [ ] Settlement is idempotent under retry
