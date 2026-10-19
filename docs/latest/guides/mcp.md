---
title: "MCP Server - PrimeNG"
type: "guide"
version: "latest"
source_url: "https://primeng.dev/llms/pages/mcp.md"
description: "Give compatible AI assistants access to PrimeNG component documentation, guides, examples, and Angular API metadata."
---

# MCP Server - PrimeNG

Give compatible AI assistants access to PrimeNG component documentation, guides, examples, and Angular API metadata.

## Claudecode

Add the PrimeNG MCP server using the CLI. After adding, start a new session and use /mcp to verify the connection.

```bash
# Add to user config (available in all projects)
claude mcp add primeng -s user -- npx -y @primeng/mcp

# Or add to current project only
claude mcp add primeng -- npx -y @primeng/mcp
```

## Cli

The recommended setup is the PrimeNG Plugin . It installs this MCP server together with the PrimeNG skills for implementation, setup, theming, migration, and troubleshooting.

```bash
pnpm dlx @primeui/cli plugin install --tool copilot --library primeng
pnpm dlx @primeui/cli doctor --tool copilot --library primeng
```

## Cursor

Create .cursor/mcp.json in your project or ~/.cursor/mcp.json for global configuration.

```typescript
{
    "mcpServers": {
        "primeng": {
            "command": "npx",
            "args": ["-y", "@primeng/mcp"]
        }
    }
}
```

## Data

The MCP package includes a generated copy of the PrimeNG documentation. This keeps the server usable without a PrimeNG source checkout and makes its answers match the installed package version. Results include a primeui://primeng resource URI and, when available, a link to the public documentation. The tools do not edit files, install dependencies, or update configuration. validate_usage checks documented Angular component metadata. If the required metadata is unavailable, it reports that limitation instead of guessing. Run your project type checks, tests, and browser checks after making changes. MCP validation does not replace them. If you installed the PrimeNG Plugin, do not register the MCP server again with a manual configuration. Run the CLI doctor command if the server is missing or appears more than once.

## Introduction

Model Context Protocol (MCP) lets AI assistants call external tools. The @primeng/mcp package uses those tools to search and read the PrimeNG documentation included with the package. Find components, guides, examples, and standalone setup instructions. Read component APIs, Angular inputs and outputs, templates, and documentation sections. Check component usage against documented selectors, inputs, and outputs. Return links to the related PrimeNG documentation when available. The server requires Node.js 22 or newer. It is read-only and does not change your project files or PrimeNG configuration.

## Openaicodex

Add the MCP server with the Codex CLI, or add the same configuration to ~/.codex/config.toml .

```bash
# Using the CLI
codex mcp add primeng -- npx -y @primeng/mcp

# Or edit ~/.codex/config.toml
[mcp_servers.primeng]
command = "npx"
args = ["-y", "@primeng/mcp"]
```

## Tools

The PrimeNG MCP server provides eight tools. Tool Purpose list List available components, guides, examples, resources, or modes. search Search generated component, guide, Angular API, section, and example text. get_component Read component documentation, Angular API metadata, and available sections. get_guide Read a generated PrimeNG guide and its source metadata. get_example Return an example from a documented component section. get_setup Return setup guidance for supported standalone Angular environments. validate_usage Validate component usage against generated PrimeNG API metadata. version Report the server version and the status of its documentation data, tools, and resources.

## Vscode

Create .vscode/mcp.json for project-level setup. To use the server across projects, add the same servers entry to your VS Code user MCP configuration.

```typescript
{
    "servers": {
        "primeng": {
            "command": "npx",
            "args": ["-y", "@primeng/mcp"]
        }
    }
}
```
