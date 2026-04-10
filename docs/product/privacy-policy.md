# Privacy Policy

## Scope

This privacy policy applies to the `codex-orchestrator` plugin in this repository.

## Data Handling

- The plugin is designed for local execution inside Codex.
- Runtime orchestration state is stored locally under `plugins/codex-orchestrator/.codex-orchestrator/state/`.
- Design specs, implementation plans, and related workflow documents are stored in this repository.
- The plugin does not include a hosted backend, external analytics pipeline, or built-in telemetry service.

## Network Access

- The plugin-local MCP server communicates over local stdio.
- Any network activity depends on the host Codex environment, user-installed tools, or repositories and services the user explicitly connects.
- This repository does not bundle a remote service that collects plugin usage events.

## User Responsibility

- Users are responsible for the repository content, credentials, and external systems they choose to connect while using Codex.
- Users should avoid storing secrets in repository files unless their own environment and workflow require it.

## Contact

For questions about this repository and plugin, use the repository issue tracker or the maintainer contact listed in the plugin manifest.
