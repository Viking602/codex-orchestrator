# Progress

## 2026-04-08

- Initialized repository routing docs and planning files.
- Captured current scope and constraints into file-backed planning artifacts.
- Added design specification and active implementation plan.
- Added artifact-model entrypoints for architecture, product, decision, and completed plan directories.
- Added category/runtime architecture documents and the file-backed execution decision record.
- Implemented the `codex-orchestrator` plugin shell, zero-third-party stdio MCP server, runtime SQLite state store, category router, structured markdown plan sync, and tests.
- Verified unit tests pass and MCP initialize/tools/list/tool-call smoke checks succeed.
- Created the phase 2 design spec and phase 2 active implementation plan.
- Switched repository routing docs to point at the phase 2 plan as the new execution anchor.
- Implemented phase 2 write lease storage and tools, strengthened watchdog recommendations, and added deterministic parent `next_action` derivation.
- Added phase 2 architecture docs and expanded tests for lease and next-action behavior.
- Released the phase 2 active write leases after implementation tasks were accepted.
- Verified phase 2 unit tests pass and the new tools are exposed over stdio MCP.
- Created the phase 3 design spec and phase 3 active implementation plan.
- Switched routing docs so phase 3 is now the active execution anchor.
- Implemented strict question gate, subagent completion assessment, deterministic review/repair stage derivation, and completion guard behavior.
- Released the phase 3 active write leases after implementation tasks were accepted.
- Verified phase 3 unit tests pass and the new tools are exposed over stdio MCP.
- Replaced scaffold placeholder plugin metadata with repository-backed manifest URLs and added repository-hosted privacy and terms documents.
- Created a bundled-agent design spec and active implementation plan for plugin-shipped Codex agent roles.
- Bundled six plugin-owned Codex agent definitions and wired them through the plugin agent manifest.
- Added bundle documentation and a regression test that keeps category preferred roles aligned with the bundled inventory.
- Generalized the bundled default implementation role to `backend-developer` so the plugin stays coding-oriented instead of specialist-oriented or language-locked.
- Created the installer design spec and installer active implementation plan.
- Switched routing docs so the installer plan is now the active execution anchor.
- Implemented the installer script with link/copy modes, dry-run, marketplace registration, and safe agent backup behavior.
- Verified installer tests pass and documented the one-click install flow in the README.

## 2026-04-11

- Created the marketplace-install design spec and active implementation plan.
- Switched routing docs so the marketplace-install plan became the execution anchor.
- Added a repo-local marketplace at `.agents/plugins/marketplace.json` so Codex can discover the plugin from the repository surface.
- Updated the installer to target the current Codex plugin model: personal plugin source path, personal marketplace entry, installed cache copy, and plugin enabled state.
- Removed forced `features.apps = true` writes from plugin installation.
- Expanded installer tests to cover repo marketplace metadata, personal marketplace bootstrap, installed cache staging, and config preservation behavior.
- Verified the installer tests pass with the new marketplace/bootstrap behavior.
- Installed the plugin into the local Windows Codex home under `%USERPROFILE%\.codex` and wrote the matching personal marketplace/config entries.
