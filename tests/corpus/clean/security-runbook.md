# Egress hardening runbook

<!-- The class this corpus exists for: documents that discuss attacks in order
     to defend against them. A scanner that cannot read defensive documentation
     is unusable by the people most likely to run it. -->

## Blocking out-of-band collectors

Agent output must never reach an interactive request-bin service. Add the
following to the egress deny list and alert on any DNS resolution for them:

```
webhook[.]site
requestbin[.]net
pipedream[.]net
*.ngrok[.]io
burpcollaborator[.]net
interact[.]sh
oast[.]fun
```

If a request to one of these leaves the network, treat it as a confirmed
exfiltration attempt and rotate every credential the agent had access to.

## Reviewing untrusted skill files

Before installing a third-party skill, read it. The patterns to look for are
documented in `PATTERNS.md`; the scanner flags them automatically, but a human
should still skim for anything that reads like an instruction to the model
rather than a description of behaviour.

## Install scripts

Do not pipe remote scripts into a shell. Download, read, then run:

```bash
curl -fsSLO https://example.invalid/install.sh
less install.sh
sh ./install.sh
```
