# Codex Orchestrator Relative Documentation Path Policy Design

## Context

The repository currently contains machine-specific absolute filesystem paths inside markdown documents and routing surfaces. These absolute links are brittle, hostile to portability, and directly contradict the goal of repository-backed documentation. A document should survive checkout location changes, operating system changes, and maintainer changes without embedding one developer's filesystem.

The policy must therefore become hard, not advisory:

- repository documentation must not contain absolute filesystem paths
- agents should repair legacy absolute-path documentation as soon as they encounter it during normal repository work
- regression coverage should fail closed when absolute paths reappear

## Goals

- Ban machine-specific absolute filesystem paths in repository documentation.
- Require repo-relative links for repository artifacts.
- Require portable placeholders or environment variables for environment examples.
- Make first-touch cleanup of legacy absolute-path docs part of normal execution, without asking the user.
- Add an automated regression test that fails when absolute filesystem paths appear in markdown docs.

## Non-goals

- Banning normal URLs such as `https://...`.
- Rewriting runtime code paths or source attribution comments outside markdown docs.
- Replacing user-facing environment-variable examples such as `$CODEX_HOME`, `${HOME}`, or `%USERPROFILE%`.

## Design

### 1. Hard Documentation Rule

Repository documentation must not contain machine-specific absolute filesystem paths such as:

- a macOS home-directory path
- a Linux home-directory path
- a mounted host path
- a Windows drive-root path

For repository references:

- use repo-relative markdown links such as `docs/index.md`, `../AGENTS.md`, or `privacy-policy.md`
- do not use repo-root-prefixed absolute links

For environment examples:

- prefer environment variables or placeholders such as `$CODEX_HOME`, `${HOME}`, `%USERPROFILE%`, or `<repo-root>`
- do not hard-code one maintainer's machine path

### 2. First-Touch Repair Rule

When an agent first intervenes in the repository and notices legacy absolute-path documentation, it should repair those docs in the same pass without asking the user for permission. This is routine hygiene, not an ambiguous product decision.

Scope:

- routing docs such as `AGENTS.md`, `docs/index.md`, and install guides
- architecture notes, product docs, and other repository markdown files touched during the task

### 3. Enforcement Surface

The hard rule should live in the repository's durable instruction and routing surfaces:

- root `AGENTS.md`
- active execution plan
- orchestrator skill guidance

This keeps the rule visible both to direct contributors and to the plugin's normal AI workflow.

### 4. Regression Test

Add a markdown regression test that scans repository docs for forbidden absolute filesystem path patterns. The test should cover:

- `AGENTS.md`
- `README.md`
- `install.md`
- `docs/**/*.md`
- orchestrator markdown skill files that participate in repo workflow guidance

The test should fail with file and line evidence so future regressions are cheap to repair.

## Success Criteria

- Routing docs and markdown docs no longer contain machine-specific absolute filesystem paths.
- Repository artifact links are repo-relative.
- Install examples use portable placeholders or environment variables instead of hard-coded machine paths.
- `AGENTS.md` and the orchestrator skill treat this as a hard rule.
- Regression tests fail closed if absolute filesystem paths reappear in markdown docs.
