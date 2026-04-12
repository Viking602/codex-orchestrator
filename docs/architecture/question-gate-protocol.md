# Question Gate Protocol

## Purpose

The question gate decides whether the parent may ask the user a follow-up question or must continue without asking.

## Default Rule

Optional expansion is not a valid reason to ask the user.

Redundant direction confirmation is also not a valid reason to ask the user.

If the user did not explicitly request the expansion:

- do not ask
- do not widen scope
- record the assumption or skip the expansion path

If the user already supplied a workable direction:

- do not ask `should I proceed`
- do not restate the chosen direction just to wait for approval
- record assumptions briefly
- plan and execute unless a hard blocker exists

## Allowed User-Facing Question Categories

- `identity`
- `credential`
- `destructive`
- `conflict`

## Non-Question Categories

- `optional_expansion`
- `direction_confirmation`
- `system`
- `none`

## MCP Behavior

`orchestrator_question_gate` returns:

- `ask_user`
- `blocker_type`
- `allowed_to_expand`
- `recommended_action`
- `reason`

For `direction_confirmation`, the expected result is:

- `ask_user = false`
- `allowed_to_expand = false`
- `recommended_action = "plan_and_execute"`

