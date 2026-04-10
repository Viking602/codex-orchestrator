# Question Gate Protocol

## Purpose

The question gate decides whether the parent may ask the user a follow-up question or must continue without asking.

## Default Rule

Optional expansion is not a valid reason to ask the user.

If the user did not explicitly request the expansion:

- do not ask
- do not widen scope
- record the assumption or skip the expansion path

## Allowed User-Facing Question Categories

- `identity`
- `credential`
- `destructive`
- `conflict`

## Non-Question Categories

- `optional_expansion`
- `system`
- `none`

## MCP Behavior

`orchestrator_question_gate` returns:

- `ask_user`
- `blocker_type`
- `allowed_to_expand`
- `recommended_action`
- `reason`

