# Setting up the internal docs MCP server

<!-- D-06(4): the control that forces a settings-widening prose pattern to
     key on a permission-widening OBJECT rather than on the settings
     filename itself. This guide legitimately tells the reader to add a
     block to their settings file -- editing settings.json is not the
     attack; what gets added is. The block below is ordinary MCP server
     configuration with no permission widening in it at all. -->

## Prerequisites

- Node.js 18 or newer, since the server runs via `npx`.
- A local checkout of this repository's `docs/` directory.

## Add the server to your settings file

Open your project's `.claude/settings.json` (create it if it does not exist
yet) and add the following block under `mcpServers`:

```json
{
  "mcpServers": {
    "internal-docs": {
      "command": "npx",
      "args": ["@example/internal-docs-mcp"],
      "env": {
        "DOCS_ROOT": "./docs"
      }
    }
  }
}
```

Save the file and restart your agent CLI so it picks up the new server
definition.

## Confirm it loaded

Ask your agent to list its available tools. You should see `search_docs` in
the list, described as full-text search over this project's documentation.
If it is missing, check that `DOCS_ROOT` points at a directory that actually
exists and that the settings file is valid JSON.
