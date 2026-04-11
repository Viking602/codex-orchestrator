# Codex Orchestrator Delegation-First Dispatch Design

## Context

The plugin already resolves task categories, preferred roles, and `next_action`, but that chain still leaves too much execution in the local parent. Category resolution tells the parent what kind of work it is, yet it does not explicitly tell the parent whether the work should stay local or be handed to a child agent. `orchestrator_next_action` also returns a generic action such as `dispatch_task` without a machine-readable delegation decision.

That gap shows up most clearly on analysis and repo-understanding requests. Even when the category resolves to `research` and the preferred role is `search-specialist`, the system still behaves as if subagent dispatch were optional parent judgment instead of a workflow default. The desired behavior is stricter: classify the task first, then decide whether subagent intervention is required, with a bias toward child execution for normal repository work.

## Goals

- Make delegation preference part of the category contract instead of leaving it implicit in prose.
- Expose a machine-readable child-intervention decision in parent-facing MCP outputs.
- Bias normal repository work toward child execution instead of local-parent execution.
- Preserve the parent as the control-plane owner for plan sync, review gating, and acceptance.

## Non-goals

- Having the MCP server call `spawn_agent` directly.
- Replacing Codex native subagent execution.
- Eliminating the parent's control-plane responsibilities.
- Designing a generic autonomous multi-agent scheduler outside the current plugin scope.

## Design

### 1. Category Contract Gains Delegation Preference

Each category definition should carry an explicit `delegation_preference` field:

- `parent-only`
- `prefer-subagent`
- `subagent-required`

This keeps the “should a child agent do the work?” decision in the same contract that already owns role, write policy, review requirements, and parallelism.

Initial policy:

- `plan` -> `prefer-subagent`
- `research` -> `subagent-required`
- `backend-impl` -> `subagent-required`
- `review` -> `subagent-required`

The parent remains local for control-plane-only actions such as acceptance, question gating, final completion, and runtime bookkeeping.

### 2. Category Resolution Should Surface Default Delegation Bias

`orchestrator_resolve_category` should return:

- `delegation_preference`
- `requires_subagent_default`

This keeps the first decision explicit: identify the task type and its default execution owner before plan-state logic enters the picture.

### 3. Next Action Should Return Child-Intervention Metadata

`orchestrator_next_action` should continue returning the current action, but it should also return:

- `requires_subagent`
- `dispatch_role`
- `intervention_reason`
- `dispatch_mode`

`dispatch_mode` is derived from category parallelism:

- `single` -> `single-subagent`
- `parallel` -> `parallel-subagents`
- `write-scope` -> `write-scope-subagent`

This gives the parent a deterministic answer to two separate questions:

1. What kind of work is this?
2. Should this step be executed by a child agent?

### 4. Parent Behavior Becomes Delegation-First

The parent should interpret the new fields as follows:

- If `requires_subagent` is `true`, the parent should dispatch or continue the indicated child role instead of doing the work locally.
- If `requires_subagent` is `false`, the parent may continue locally because the step is control-plane work.
- `continue_same_agent` should preserve the existing child assignment when one already exists.
- Lease acquisition, review recording, plan synchronization, and task acceptance remain parent-owned even when implementation work is child-owned.

### 5. Compatibility Strategy

The plugin should not require a new orchestration tool just to answer the delegation question. The existing control-plane path remains:

1. `orchestrator_resolve_category`
2. `orchestrator_next_action`

The change is that both tools now expose delegation intent explicitly instead of leaving it buried in role names and prose.

## Success Criteria

- Category definitions include delegation preference.
- `orchestrator_resolve_category` exposes default delegation bias.
- `orchestrator_next_action` exposes child-intervention metadata.
- Tests cover category-level delegation preference and next-action subagent decisions.
- Architecture docs and the orchestrator skill explain that normal repository work is delegation-first while the parent remains the control plane.
