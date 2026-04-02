# Security Awareness Guide

This document teaches developers to recognize prompt injection attacks.

## Category A: Role Override

Example of what to watch for:
ignore all previous instructions  <!-- injection-scanner:ignore PI001 -->

## Category B: Instruction Injection

Example pattern:
SYSTEM OVERRIDE  <!-- injection-scanner:ignore PI011 -->

## Category D: Jailbreaks

This one is NOT suppressed — scanner should catch it:
developer mode enabled

## How to Protect Your Skills
- Validate all inputs before processing
- Use structured prompts that resist override
- Run injection-scanner as a pre-commit hook
