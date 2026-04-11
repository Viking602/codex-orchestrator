# Codex Orchestrator Install Guide Design

## Context

The repository already has a working installer, bootstrap README instructions, and local verification paths for marketplace registration, cache staging, global `AGENTS` routing, and plugin enablement. What it does not have is a single AI-oriented entrypoint that tells a shell-capable coding agent exactly how to install the plugin itself without asking the user to edit files manually.

That gap matters because installation is now part of the plugin's expected self-serve workflow. The repository needs a dedicated `install.md` that is written for an agent, not just for a human browsing the README.

## Goals

- Add a root-level `install.md` that tells an AI agent how to install `codex-orchestrator` end to end.
- Keep the guide aligned with the supported installer contract instead of hand-edited config steps.
- Include verification commands so the agent can prove installation state with evidence.
- Make the new guide discoverable from the main routing surfaces.

## Non-goals

- Replacing the existing installer script.
- Adding a second installation path.
- Documenting unsupported manual edits as the default workflow.

## Design

### File Placement

The install guide should live at the repository root as `install.md`. This makes it easy for both humans and agents to discover before entering deeper documentation trees.

### Audience And Tone

The guide should speak directly to an AI agent with shell access. It should explicitly instruct the agent to run the installer and verification commands itself instead of asking the user to hand-edit files.

### Required Sections

The guide should include:

- purpose and intended audience
- prerequisites
- standard install command
- optional `--link` and `--dry-run` variants
- Windows and WSL note for `%USERPROFILE%`
- files the installer writes
- verification commands for plugin cache, `config.toml`, and global `AGENTS`
- a fresh-process validation command using `codex exec`
- restart requirement for already-running desktop sessions
- note about repo-local `AGENTS.md` in other repos overriding the global default workflow

### Routing Updates

The new guide should be referenced from:

- `README.md`
- `AGENTS.md`
- `docs/index.md`
- `task_plan.md`

### Verification

A regression test should verify that `install.md` exists and contains the supported install command plus key verification targets. The existing installer test suite should continue to pass after the documentation changes.

## Success Criteria

- `install.md` exists at the repository root.
- The guide tells an AI agent to run the installer and verification commands itself.
- Routing docs point at the new install guide.
- Tests cover the new guide's presence and critical references.
