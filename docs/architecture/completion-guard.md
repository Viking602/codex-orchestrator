# Completion Guard

## Purpose

The completion guard prevents the parent from ending work before plan completion reaches 100 percent.

## Rule

The parent must not finish when any of these remain open:

- unchecked top-level task in the TODO list
- unchecked item in `Final Acceptance`
- runtime blocker that still prevents plan closure

## MCP Behavior

`orchestrator_completion_guard` returns:

- `can_finish`
- `open_tasks`
- `open_acceptance_items`
- `blocking_reason`

If `can_finish = false`, the parent must continue execution or explicitly report the blocker. It must not present the work as complete.

