# Codex Orchestrator Default Workflow Routing Design

## Context

The plugin is now installable through Codex's marketplace-driven local plugin model, but installation alone does not make Codex route repository work through the plugin by default. Current Codex plugin docs say installation is followed by either asking Codex to use the plugin in a new thread or explicitly invoking the plugin or one of its bundled skills with `@`. Current Codex customization docs also say that implicit skill choice depends on skill metadata, while `AGENTS.md` files participate in the session instruction chain.

That means the current repository still has a routing gap: `codex-orchestrator` is installed and enabled, but generic process skills can still win the initial route unless the user manually invokes the plugin. The repository needs a durable default-routing layer so Codex treats `codex-orchestrator` more like the installed superpowers workflow: the default repo-task workflow, not an optional extra the user has to remember to mention every time.

## Goals

- Make `codex-orchestrator` the default workflow for repository tasks when it is installed and enabled.
- Drive that default through durable instruction surfaces instead of relying on users to type `@codex-orchestrator`.
- Strengthen bundled skill metadata so implicit discovery matches common repository tasks more reliably.
- Keep the bootstrap idempotent and installer-driven.
- Preserve an escape hatch when the user explicitly asks for a different workflow or when the plugin is unavailable.

## Non-goals

- Forcing `codex-orchestrator` onto non-repository chat.
- Disabling generic process skills globally.
- Requiring live mutation of already running threads.
- Replacing domain skills such as framework- or language-specific implementation guides.

## Design

### Routing Surfaces

Default routing should be established through four coordinated surfaces:

1. Global Codex guidance: a managed block in the active global `AGENTS` file.
2. Repository guidance: explicit repo-local routing instructions in the root `AGENTS.md`.
3. Bundled skill metadata: broader trigger wording in the plugin skill so implicit discovery matches real task language.
4. Install-surface metadata: plugin UI text and starter prompts should describe the plugin as the default repository workflow, but this remains supportive metadata rather than the primary routing contract.

### Global AGENTS Bootstrap

The installer should write a managed `codex-orchestrator` block into the active global guidance file:

- If `~/.codex/AGENTS.override.md` exists, update that file.
- Otherwise, update `~/.codex/AGENTS.md`.

The managed block must be idempotent and replaceable through explicit markers so repeated installs update the same section instead of appending duplicates. Existing user guidance must be preserved outside the managed block.

The block should tell Codex:

- use `codex-orchestrator` as the default workflow for repository tasks when installed and enabled
- start with the bundled `codex-orchestrator` skill for feature work, bug fixes, refactors, debugging, docs changes, architecture work, and other multi-step repo tasks
- treat generic process skills as subordinate helpers after orchestration begins or when the plugin is unavailable
- follow stronger repo-local guidance when a repository provides it

### Repository Guidance

The root `AGENTS.md` should explicitly say that `codex-orchestrator` is the default workflow for this repository and should be invoked before generic process skills. This repo-level instruction is closer to the working directory than the global guidance and should therefore refine the default behavior for this repository specifically.

The repo guidance should stay routing-oriented:

- route repo tasks into `codex-orchestrator`
- require the active plan as the execution source of truth
- keep routing docs synchronized when paths or entrypoints change

### Bundled Skill Discovery

The bundled skill description should be widened from orchestration-specific jargon to common task language. It should match prompts about:

- feature work
- bug fixes
- refactors
- debugging
- architecture changes
- docs changes
- multi-step repository work

The body should also make the ownership boundary explicit: once selected, `codex-orchestrator` owns spec, plan, category routing, runtime state, and review gates, while generic process skills become helpers rather than competing top-level workflows.

### Install-Surface Metadata

The plugin manifest should describe the plugin as the default repository workflow for planning, execution tracking, and review gates. `defaultPrompt` text should reinforce that positioning for composer UX, but this metadata should not be treated as the actual enforcement mechanism.

### Installer Contract

The installer gains one more bootstrap responsibility in addition to marketplace, plugin, cache, and config setup:

- write or update the managed global `AGENTS` block

The script should expose an override flag for tests and custom environments:

- `--global-agents-path <file>`

Dry-run mode must report the intended guidance update without writing anything.

### Runtime Expectations

This design does not attempt to hot-patch existing running conversations. After installer-driven routing changes, Codex should be restarted before expecting the new default workflow to apply consistently. New threads created after restart are the verification target.

## Success Criteria

- The installer writes an idempotent managed block into the active global `AGENTS` file.
- Root `AGENTS.md` explicitly routes repository tasks into `codex-orchestrator`.
- The bundled skill description matches common repository-task language instead of only orchestration-specific wording.
- Installer tests cover global `AGENTS` bootstrap behavior and preservation of pre-existing guidance.
- A fresh Codex run can discover the stronger routing instructions without requiring manual `@` invocation.
