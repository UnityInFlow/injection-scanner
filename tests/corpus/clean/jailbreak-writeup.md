# Jailbreak techniques — a reviewer's field guide

<!-- Added in #99. The nearest legitimate document to a jailbreak payload is
     security writing ABOUT jailbreaks, and this repo publishes three such files
     (PATTERNS.md, docs/DETECTION-BACKLOG.md, this corpus's security-runbook.md).
     The old patterns had this exactly backwards: PI039 matched the string
     "grandma exploit" — the technique's NAME — and missed the payload the
     technique consists of. So it fired on write-ups like this one and not on
     attacks. Every line below names a technique; none of them is an attempt to
     perform one. -->

## Persona attacks

The best-known family. An attacker assigns the model an alternative identity that
supposedly lacks the original's limits — DAN is the canonical example, and AIM,
STAN and DUDE are variations on it. Naming the persona is not the attack; the
attack is the sentence assigning it.

Detection here is genuinely hard, because a persona instruction and a legitimate
role scoping ("you are a code reviewer") have the same grammar.

## The grandmother framing

A social-engineering wrapper: the request is embedded in a sentimental memory so
that refusing appears unkind. Widely known as the grandma exploit. It is worth
understanding because it generalises — any framing that makes refusal feel like a
social failure works the same way.

## Prefill and continuation

The attacker supplies the opening of the response so the model continues rather
than deciding. Cheap to attempt and cheap to detect, since the tell is an
instruction about how the reply must begin.

## Hypothetical and research framing

"For research purposes" and "hypothetically speaking" both try to move the
request into a frame where the usual answer supposedly does not apply. Security
training material uses the same words honestly, which is why our pattern for this
family is graded MEDIUM rather than HIGH.

## Nested simulation

The model is asked to simulate another system that has no restrictions, putting a
layer of indirection between the request and the refusal. Compare the developer
mode and unlimited mode claims, which assert a privileged state directly.

## Review guidance

When you see a jailbreak prompt in a pull request, check whether it is a payload
or a description of one. This file is a description. `examples/jailbreak-attack.md`
is a payload, and it is stored there deliberately so that the scanner has
something to find.
