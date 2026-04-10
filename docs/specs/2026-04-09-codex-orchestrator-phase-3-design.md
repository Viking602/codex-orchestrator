# Codex Orchestrator Plugin Phase 3 Design

## Context

Phase 1 established the file-backed orchestrator shell. Phase 2 hardened the control plane with write leases, stronger watchdog hints, and deterministic `next_action` derivation. Phase 3 focuses on parent-agent accountability: the parent should not accept partial child work, should not ask expansion questions by default, should automatically drive review and repair loops, and should be unable to declare the run complete before plan completion reaches 100%.

## Goals

- Add a strict question gate that rejects user-facing questions for optional scope expansion.
- Add subagent completion assessment so implementer output is judged against task steps, review state, and evidence, not by self-report.
- Add automatic review and repair loop guidance so parent orchestration always knows whether to review, repair, redispatch, or accept.
- Add a completion guard that fails closed when unfinished work remains.

## Non-goals

- Fully autonomous background daemons that dispatch subagents without parent calls.
- Replacing Codex native subagent execution.
- Solving every domain-specific repair strategy in phase 3.

## Design Principles

### 1. Parent Owns Outcome, Not The Child

An implementer returning `DONE` means only that implementation has paused. It does not imply:

- all required steps were completed
- evidence exists
- review passed
- task is acceptable

The parent must pass the child output through explicit completion assessment.

### 2. Optional Expansion Must Not Trigger Questions

The default rule for new optional work is:

- do not ask the user
- do not widen scope
- record assumption or ignore the expansion path

Only identity, credential, destructive, or true instruction-conflict blockers should surface as user questions.

### 3. Review Is Mandatory And Repair Loops Are Structural

After implementation, the system should automatically move into:

- spec review
- quality review
- repair loop if either fails

The parent should not improvise this loop from prose. MCP tools should derive the next required stage.

### 4. Completion Guard Must Fail Closed

If the parent asks whether work can finish, the answer must be `no` unless:

- all top-level tasks are accepted
- all final acceptance checkboxes are checked
- no active blockers remain
- no active lease-required implementation tasks remain open

## New Runtime Concepts

### Question Gate Result

- `ask_user`
- `blocker_type`
- `reason`
- `allowed_to_expand`
- `recommended_action`

### Completion Assessment Result

- `task_id`
- `implementation_complete`
- `missing_steps`
- `missing_evidence`
- `next_required_stage`
- `repair_role`
- `can_accept`

### Completion Guard Result

- `can_finish`
- `open_tasks`
- `open_acceptance_items`
- `blocking_reason`

## New MCP Tools

- `orchestrator_question_gate`
- `orchestrator_assess_subagent_completion`
- `orchestrator_completion_guard`

## Success Criteria

- Optional expansion no longer becomes a user-facing question by default.
- Child implementer output cannot be treated as task completion without assessment.
- Parent can deterministically enter review or repair loops after child output.
- Completion guard returns `can_finish = false` whenever plan completion is below 100%.

