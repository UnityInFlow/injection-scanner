## API

### Tools

- **read_text_file**
  - Read complete contents of a file as text
  - Inputs:
    - `path` (string)
    - `head` (number, optional): First N lines
    - `tail` (number, optional): Last N lines
  - Always treats the file as UTF-8 text regardless of extension
  - Cannot specify both `head` and `tail` simultaneously

- **read_media_file**
  - Read a file and return it as a base64-encoded content block with its MIME type
  - Inputs:
    - `path` (string)
  - Streams the file and returns base64 data with the corresponding MIME type. Image and
    audio files are returned as `image`/`audio` content; any other file type is returned as
    an embedded `resource` (a valid MCP content block for arbitrary binary data)

- **read_multiple_files**
  - Read multiple files simultaneously
  - Input: `paths` (string[])
  - Failed reads won't stop the entire operation

- **write_file**
  - Create new file or overwrite existing (exercise caution with this)
  - Inputs:
    - `path` (string): File location
    - `content` (string): File content

- **edit_file**
  - Make selective edits using advanced pattern matching and formatting
  - Features:
    - Line-based and multi-line content matching
    - Whitespace normalization with indentation preservation
    - Multiple simultaneous edits with correct positioning
    - Indentation style detection and preservation
    - Git-style diff output with context
    - Preview changes with dry run mode
  - Inputs:
    - `path` (string): File to edit
    - `edits` (array): List of edit operations
      - `oldText` (string): Text to search for (can be substring)
      - `newText` (string): Text to replace with
    - `dryRun` (boolean): Preview changes without applying (default: false)
  - Returns detailed diff and match information for dry runs, otherwise applies changes
  - Best Practice: Always use dryRun first to preview changes before applying them

- **create_directory**
  - Create new directory or ensure it exists
  - Input: `path` (string)
  - Creates parent directories if needed
  - Succeeds silently if directory exists

- **list_directory**
  - List directory contents with [FILE] or [DIR] prefixes
  - Input: `path` (string)

- **list_directory_with_sizes**
  - List directory contents with [FILE] or [DIR] prefixes, including file sizes
  - Inputs:
    - `path` (string): Directory path to list
    - `sortBy` (string, optional): Sort entries by "name" or "size" (default: "name")
  - Returns detailed listing with file sizes and summary statistics
  - Shows total files, directories, and combined size

- **move_file**
  - Move or rename files and directories
  - Inputs:
    - `source` (string)
    - `destination` (string)
  - Fails if destination exists

- **search_files**
  - Recursively search for files/directories that match or do not match patterns
  - Inputs:
    - `path` (string): Starting directory
    - `pattern` (string): Search pattern
    - `excludePatterns` (string[]): Exclude any patterns.
  - Glob-style pattern matching
  - Returns full paths to matches

- **directory_tree**
  - Get recursive JSON tree structure of directory contents
  - Inputs:
    - `path` (string): Starting directory
    - `excludePatterns` (string[]): Exclude any patterns. Glob formats are supported.
  - Returns:
    - JSON array where each entry contains:
      - `name` (string): File/directory name
      - `type` ('file'|'directory'): Entry type
      - `children` (array): Present only for directories
        - Empty array for empty directories
        - Omitted for files
  - Output is formatted with 2-space indentation for readability
    
- **get_file_info**
  - Get detailed file/directory metadata
  - Input: `path` (string)
  - Returns:
    - Size
    - Creation time
    - Modified time
    - Access time
    - Type (file/directory)
    - Permissions

- **list_allowed_directories**
  - List all directories the server is allowed to access
  - No input required
  - Returns:
    - Directories that this server can read/write from

### Tool annotations (MCP hints)

This server sets [MCP ToolAnnotations](https://modelcontextprotocol.io/specification/2025-03-26/server/tools#toolannotations)
on each tool so clients can:

- Distinguish **read‑only** tools from write‑capable tools.
- Understand which write operations are **idempotent** (safe to retry with the same arguments).
- Highlight operations that may be **destructive** (overwriting or heavily mutating data).
- Signal that a tool does **not** reach an open or external world (every filesystem tool sets `openWorldHint: false`).

The mapping for filesystem tools is:

| Tool                        | readOnlyHint | idempotentHint | destructiveHint | Notes                                            |
|-----------------------------|--------------|----------------|-----------------|--------------------------------------------------|
| `read_text_file`            | `true`       | –              | –               | Pure read                                       |
| `read_media_file`           | `true`       | –              | –               | Pure read                                       |
| `read_multiple_files`       | `true`       | –              | –               | Pure read                                       |
| `list_directory`            | `true`       | –              | –               | Pure read                                       |
| `list_directory_with_sizes` | `true`       | –              | –               | Pure read                                       |
| `directory_tree`            | `true`       | –              | –               | Pure read                                       |
| `search_files`              | `true`       | –              | –               | Pure read                                       |
| `get_file_info`             | `true`       | –              | –               | Pure read                                       |
| `list_allowed_directories`  | `true`       | –              | –               | Pure read                                       |
| `create_directory`          | `false`      | `true`         | `false`         | Re‑creating the same dir is a no‑op             |
| `write_file`                | `false`      | `true`         | `true`          | Overwrites existing files                       |
| `edit_file`                 | `false`      | `false`        | `true`          | Re‑applying edits can fail or double‑apply      |
| `move_file`                 | `false`      | `false`        | `true`          | Deletes source file                             |

> Note: `idempotentHint` and `destructiveHint` are meaningful only when `readOnlyHint` is `false`, as defined by the MCP spec. Every tool also sets `openWorldHint: false` — this server only accesses the local filesystem within its allowed directories, never an open or external world.

