# Egress hardening runbook

<!-- The class this corpus exists for: documents that discuss attacks in order
     to defend against them. A scanner that cannot read defensive documentation
     is unusable by the people most likely to run it.

     The "Install scripts" section additionally holds PI028's false-positive
     control (#122). PI028 was widened to accept an interpreter named by an
     absolute path (`| /bin/sh`, `| /usr/bin/env sh`); the mutation that
     widening invites is to let the pipe target be anything at all. Before
     #122 that mutation left this whole corpus green -- nothing here piped a
     fetch into a non-shell filter -- so the mutation was caught only by
     per-pattern negatives, which is the weaker of the two gates. The three
     `curl ... | jq / sha256sum / tar` lines below now make the corpus itself
     go red under it (4 strict matches), and the two bare `/usr/bin/env sh` /
     `#!/bin/bash` mentions do the same for the neighbouring mutation that
     drops the fetch-and-pipe requirement and keys on the interpreter alone. -->

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

Piping a fetch into a *non-executing* filter is ordinary and stays allowed —
verifying a checksum or reading one field out of a release manifest is how the
download-then-review step above is automated, and neither hands the remote
bytes to an interpreter:

```bash
curl -fsSL https://example.invalid/release.json | jq -r '.version'
curl -fsSL https://example.invalid/install.sh | sha256sum -c ./install.sh.sum
curl -fsSL https://example.invalid/bundle.tar.gz | tar -tzf -
```

Naming an interpreter is likewise not the same as feeding one. Our hook wrapper
is executed with `/usr/bin/env sh` rather than the operator's login shell, and
the vendored helpers under `tools/` still declare `#!/bin/bash`; both are facts
about where a *reviewed, local* script runs.
