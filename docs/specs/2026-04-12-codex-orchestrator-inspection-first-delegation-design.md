# Codex Orchestrator Inspection-First Delegation Design

## Context

`codex-orchestrator` already encodes a subagent-first contract for `research`, `backend-impl`, and `review`.

However, repository inspection requests still fall through too often because the entry classification is narrower than the language users actually use.

In particular:

- `research` routing currently depends on a short English-only keyword list such as `inspect`, `scan`, and `research`
- users often ask for repository inspection in broader language such as `check this codebase`, `look through the repo`, `audit the project`, or Chinese prompts such as `检查某个项目的代码库`
- when that wording misses the `research` classifier, the task defaults to `backend-impl`
- once the task lands in `backend-impl`, the parent can stay on the critical path instead of dispatching a read-only search subagent first

That creates the exact failure mode the user called out: codebase-check and repo-inspection requests are handled locally by the parent when they should first be routed to a search-oriented child agent.

## Goals

- Make repository inspection, codebase triage, repo scanning, and read-only audit requests classify into `research` more reliably.
- Treat those requests as search-specialist-owned work by default instead of parent-local analysis.
- Preserve the existing category contract where `research` is `subagent-required` and `parallel`.
- Keep the implementation lightweight: improve the entry heuristics and workflow guidance without adding a new MCP tool.

## Non-goals

- Replacing the existing `research` category semantics or preferred role.
- Turning every use of `check` or `review` into `research` regardless of surrounding context.
- Reworking the full next-action state machine when the current gap is entry classification.
- Adding repository-specific language packs or an LLM classifier inside the MCP runtime.

## Design

### 1. Expand Repository-Inspection Classification

The category resolver should recognize a broader set of signals for `research`.

The resolver should treat a request as `research` when it contains:

- existing research verbs such as `research`, `analyze`, `analysis`, `inspect`, `scan`, `investigate`, and `map`
- repository-inspection phrases such as `codebase`, `repo`, `repository`, `audit`, `look through`, `read through`, `understand the project`, `search the codebase`, and `triage the repo`
- Chinese repository-inspection prompts such as `检查`, `排查`, `梳理`, `审查`, `代码库`, `仓库`, `项目代码`, and `源码`

The matching does not need to be full natural-language parsing. A stronger phrase and keyword set is sufficient so long as:

- obvious inspection prompts no longer fall through to `backend-impl`
- review-gate prompts still prefer `review`
- planning prompts still prefer `plan`

### 2. Keep Inspection Work In Research

Once matched, inspection-style requests should continue to inherit the existing `research` category contract:

- preferred role: `search-specialist`
- delegation preference: `subagent-required`
- parallelism: `parallel`
- write policy: `read-only`

This preserves the correct behavior without inventing a separate `inspection` category.

### 3. Harden Workflow Guidance

The bundled orchestrator skill and repo guidance should say explicitly that:

- repository inspection and codebase-check tasks are research-stage work
- the parent should dispatch `search-specialist` first for read-only repo evidence gathering
- the parent should not keep routine codebase inspection on its own critical path unless the user explicitly asked to avoid delegation

This closes the gap between the MCP contract and the prompt-level workflow the parent follows.

### 4. Add Regression Coverage

Rust tests should fail if repository-inspection prompts regress back to the default implementation category.

Coverage should include:

- English inspection wording such as `check this codebase`
- repository-language wording such as `audit the repository`
- Chinese wording such as `检查这个项目的代码库`
- next-action metadata for `research` showing `requires_subagent = true`, `dispatch_role = search-specialist`, and `dispatch_mode = parallel-subagents`

## Verification Strategy

Verification should cover:

1. category-resolution tests for repository-inspection prompts
2. repo-contract tests for workflow text that says inspection requests go through `search-specialist`
3. full Cargo test run for the Rust runtime and repo-contract suite

## Success Criteria

- Repository inspection requests such as `check this codebase` and `检查这个项目的代码库` resolve to `research`.
- `research` next-action payloads still require subagent execution and point at `search-specialist`.
- The bundled orchestrator skill explicitly treats codebase inspection as child-owned research work.
- Cargo-based validation passes after the routing hardening.
