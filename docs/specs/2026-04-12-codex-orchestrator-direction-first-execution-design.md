# Codex Orchestrator Direction-First Execution Design

## Context

`codex-orchestrator` already absorbs repository brainstorming and pushes repository work into file-backed specs, plans, and delegated execution.

However, the workflow still over-asks in one common case:

- the user already states a workable direction or target architecture
- the agent still reopens the choice set with 2-3 approaches
- the agent then asks for another approval such as `if you agree, I will start`
- planning and execution are delayed even though no hard blocker exists

That creates a bad control-plane behavior: the user has already chosen a direction, but the workflow acts as if it still needs permission to begin.

## Goals

- When the user already supplied a workable direction, stop asking redundant confirmation questions.
- Keep clarification and approval only for genuinely missing constraints, conflicts, destructive operations, credentials, or material design forks.
- Allow the parent to summarize assumptions briefly, write the spec and active plan, and begin execution without a second `shall I proceed` turn.
- Preserve the stronger discipline around file-backed planning, review gates, and delegated execution.

## Non-goals

- Removing clarification entirely.
- Removing approach comparison when the direction is genuinely open.
- Allowing expansion into extra scope the user did not ask for.
- Weakening hard-blocker categories such as `identity`, `credential`, `destructive`, or `conflict`.

## Design

### 1. Make Direction Clarity A First-Class Workflow Rule

The bundled workflow should distinguish between two cases:

1. direction still open
2. direction already provided

When the direction is still open:

- ask clarifying questions one at a time
- compare 2-3 approaches with a recommendation
- obtain approval before freezing the implementation plan

When the user already provided a workable direction and there is no hard blocker:

- do not reopen the choice set just to show optional alternatives
- do not ask a second confirmation question to start
- summarize assumptions briefly
- write the spec and active plan
- begin execution

### 2. Add A Non-Question Gate For Redundant Re-Confirmation

`orchestrator_question_gate` should explicitly treat redundant confirmation as a non-question category.

Add a new category:

- `direction_confirmation`

Its behavior should be:

- `ask_user = false`
- `allowed_to_expand = false`
- `recommended_action = "plan_and_execute"`

This gives the parent a deterministic answer when it is tempted to ask:

- `Should I start?`
- `If this direction looks good, I will continue`
- `Do you want me to proceed with the approach you already requested?`

### 3. Narrow The Comparison-And-Approval Contract

The orchestrator skill, repo `AGENTS.md`, install-managed guidance, and plugin metadata should all say the same thing:

- compare approaches only when the direction is materially open or the user explicitly asked for options
- ask for approval before the implementation plan only when the chosen direction would materially change or reinterpret the user's request
- if the user already set the direction and no hard blocker exists, proceed directly into spec, plan, and execution

### 4. Keep The Safety Boundaries

The direction-first rule must not bypass real blockers.

The parent may still ask the user when the gate reports:

- `identity`
- `credential`
- `destructive`
- `conflict`

Optional expansion remains blocked by default.

### 5. Add Regression Coverage

Regression coverage should fail if the current workflow drifts back toward redundant re-confirmation.

Tests should cover:

- `orchestrator_question_gate` with `direction_confirmation`
- skill and guidance wording that says direction-clear requests should proceed without a second confirmation
- plugin manifest prompts that tell the host to start once direction is clear

## Verification Strategy

Verification should cover:

1. Rust runtime tests for `question_gate`
2. repo-contract tests for workflow wording and manifest prompts
3. a full Cargo test run
4. fresh installed-plugin probing so local Codex sessions pick up the new behavior

## Success Criteria

- The workflow no longer asks `shall I proceed` when the user already gave a workable direction.
- `orchestrator_question_gate` returns a plan-and-execute action for redundant direction confirmation.
- Skill text, repo guidance, install guidance, and plugin metadata all describe the same direction-first behavior.
- Cargo validation passes and the installed plugin is refreshed.
