# Codex Orchestrator Codex-Guided Install Design

## Context

The repository currently treats `scripts/install-codex-orchestrator.sh` as the supported installation path. That is no longer the desired workflow. The user wants Codex itself to perform installation and updates directly from a written guide, rather than delegating to a repository-owned shell wrapper.

The repository therefore needs to remove the shell-script install flow from current guidance, replace it with a Codex-oriented install/update guide, and keep regression coverage focused on the guide contract rather than the deleted wrapper script.

## Goals

- Remove the shell-script installer from the current install flow.
- Make `install.md` the direct Codex-runbook for installation and updates.
- Tell Codex exactly which files and directories to create, copy, update, or verify.
- Preserve the existing installation targets: plugin source, plugin cache, marketplace entry, plugin enabled state, bundled agents, and global `AGENTS` guidance.
- Keep the install and update flow idempotent at the documentation contract level.

## Non-goals

- Replacing historical design or completed-plan documents that accurately describe the former shell-script workflow.
- Changing the plugin packaging model or marketplace metadata format.
- Adding a new replacement wrapper script.

## Design

### 1. Remove The Repository-Owned Shell Installer

Delete `scripts/install-codex-orchestrator.sh` from the supported flow and stop routing users or agents toward it from current docs.

Historical documents may continue to mention it as part of the repository's past implementation record, but active guidance should not present it as the installation mechanism.

### 2. Promote `install.md` Into A Direct Codex Runbook

`install.md` should now speak directly to Codex with no intermediate installer abstraction.

The guide should tell Codex to:

- determine the active Codex home
- copy or sync `plugins/codex-orchestrator/` into the user plugin source path
- stage the installed cache copy under the local marketplace cache root
- ensure the personal marketplace entry exists
- ensure plugin enablement exists in `config.toml`
- ensure the active global `AGENTS` file contains the managed default-workflow block
- copy or refresh bundled agents into the Codex agent directory
- verify all affected files after install or update

### 3. Support Both Install And Update In The Same Guide

The guide should describe installation and update as the same idempotent reconciliation workflow:

- create missing targets on first install
- overwrite or refresh stale targets on update
- preserve user content outside the managed `codex-orchestrator` block in global `AGENTS`

### 4. Keep Verification Explicit

The guide should continue to give Codex concrete verification steps for:

- plugin cache/plugin manifest presence
- marketplace entry content
- plugin enabled state
- managed global `AGENTS` block
- bundled agent presence
- fresh-process validation where relevant

### 5. Replace Installer Tests With Guide-Contract Tests

The old installer test suite should be replaced by regression tests that verify:

- `install.md` exists
- the guide describes Codex-driven install and update
- the guide references the expected target paths
- the guide no longer presents `scripts/install-codex-orchestrator.sh` as the supported flow
- README routes to `install.md` and no longer points to the shell installer

## Success Criteria

- Current installation guidance no longer routes through `scripts/install-codex-orchestrator.sh`.
- `install.md` tells Codex how to install and update the plugin directly.
- README points to the Codex-guided install flow.
- The shell installer file is removed from the active repository surface.
- Regression tests cover the new guide contract and pass.
