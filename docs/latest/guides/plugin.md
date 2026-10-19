---
title: "Plugin - PrimeNG"
type: "guide"
version: "latest"
source_url: "https://primeng.dev/llms/pages/plugin.md"
description: "Add PrimeNG workflow skills and the PrimeNG MCP server to your AI assistant with one installation."
---

# Plugin - PrimeNG

Add PrimeNG workflow skills and the PrimeNG MCP server to your AI assistant with one installation.

## Claudecode

Add the PrimeUI marketplace, then install only the PrimeNG plugin.

```bash
claude plugin marketplace add primefaces/primeui-plugins
claude plugin install primeng@primeui
claude plugin list --json
```

## Cli

Set --tool to the assistant you use. Keep --library primeng when you run the command outside a project or from a project that uses more than one PrimeUI library.

```bash
pnpm dlx @primeui/cli plugin install --tool copilot --library primeng
```

## Codex

Add the PrimeUI marketplace, inspect its catalog, then install only the PrimeNG plugin. Use the interactive plugin browser to manage enablement.

```bash
codex plugin marketplace add primefaces/primeui-plugins
codex plugin list --available
codex plugin add primeng@primeui
codex plugin list
```

## Contents

Installing the PrimeNG Plugin adds: Seven skills for PrimeNG implementation, setup, theming, accessibility, migration, and troubleshooting. The PrimeNG MCP server, configured with @primeng/mcp . The manifest required by the selected assistant. PrimeVue and PrimeReact are separate plugins. Installing this plugin does not change your application or install PrimeNG dependencies in the project.

## Cursor

For local setup, link the PrimeNG plugin directory into Cursor's local-plugin directory. Restart Cursor or run Developer: Reload Window after creating the link.

```bash
mkdir -p ~/.cursor/plugins/local
ln -s ~/primeui-plugins/plugins/primeng ~/.cursor/plugins/local/primeng
```

## Gemini

Validate the PrimeNG extension directory, then install it from the persistent checkout. Do not install from the repository root because it contains plugins for all three PrimeUI libraries.

```bash
gemini extensions validate ~/primeui-plugins/plugins/primeng
gemini extensions install ~/primeui-plugins/plugins/primeng --consent
gemini extensions list --output-format json
```

## Introduction

The PrimeNG Plugin adds PrimeNG skills and the PrimeNG MCP server to Claude Code, Codex, GitHub Copilot, Cursor, or Gemini CLI. Plugins installed through GitHub Copilot CLI are also available in VS Code. Install the plugin when you want the assistant to follow PrimeNG-specific Angular workflows as well as read the current component documentation. The plugin is published from the primefaces/primeui-plugins repository. Use @primeui/cli for the same installation flow across supported assistants, or follow the manual setup for your client.

## Lifecycle

Use the same CLI to check the installation, update the plugin, or remove it. Each command applies only to PrimeNG in the selected assistant. Run doctor when the plugin or MCP server is not working as expected. It checks the installation, starts the MCP server, confirms the available tools, and tests documentation retrieval and component validation. Checks that an assistant does not expose are reported as unsupported.

```bash
pnpm dlx @primeui/cli plugin status --tool copilot --library primeng
pnpm dlx @primeui/cli plugin update --tool copilot --library primeng
pnpm dlx @primeui/cli plugin remove --tool copilot --library primeng
pnpm dlx @primeui/cli doctor --tool copilot --library primeng

# Machine-readable diagnostics
pnpm dlx @primeui/cli doctor --tool copilot --library primeng --json
```

## Manualsource

Claude Code, Codex, and GitHub Copilot can install the plugin directly from the public marketplace. For manual Cursor or Gemini CLI setup, clone the repository to a location you intend to keep because those clients use the selected plugins/primeng directory after installation.

```bash
git clone --branch main https://github.com/primefaces/primeui-plugins.git ~/primeui-plugins
ls ~/primeui-plugins/plugins/primeng
```

## Skills

Each skill covers one type of PrimeNG work. The skills use @primeng/mcp when they need component APIs, examples, setup guides, or usage validation. Skill Use it for primeng-router Choose the PrimeNG skill that matches the task. primeng-component-implementation Choose and implement components, templates, directives, forms, inputs, and outputs. primeng-setup-installation Configure the supported standalone Angular path with ApplicationConfig , app.config.ts , and providePrimeNG . primeng-theming-customization Configure presets, tokens, styled or unstyled mode, Tailwind, pass-through, and component styles. primeng-accessibility-icons Review semantics, keyboard behavior, focus, accessible names, PrimeIcons, and custom icons. primeng-migration Plan and apply changes using the current PrimeNG migration guides. primeng-audit-troubleshooting Find invalid APIs and troubleshoot components, setup, MCP, plugin, or duplicate configurations. Skill names use the primeng- prefix because assistants may keep skills from several plugins in one shared namespace.

## Skillworkflow

You do not need to select a skill manually. Describe the PrimeNG task and the router chooses the relevant workflow. The router checks that the project uses Angular and PrimeNG. The selected skill reads only the component or guide information needed for the task. When component metadata is available, the skill validates the final PrimeNG usage. If the documentation does not confirm a setup or API, the skill reports the limitation instead of guessing. After implementation, run the checks required by your project, including type checking, tests, browser behavior, accessibility, and visual review where applicable.

## Vscodecopilot

Install the PrimeNG Plugin through GitHub Copilot CLI. VS Code automatically discovers plugins installed by Copilot CLI, so it does not need a separate plugin configuration.

```bash
copilot plugin marketplace add primefaces/primeui-plugins
copilot plugin install primeng@primeui
```
