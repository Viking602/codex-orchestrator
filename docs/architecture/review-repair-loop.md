# Review Repair Loop

## Purpose

The parent must always route child output through review and, when necessary, back into repair before acceptance.

## Rules

1. Child `DONE` is not task completion.
2. Parent runs completion assessment first.
3. If assessment is incomplete, return to implementation.
4. If assessment is complete, run spec review.
5. If spec review fails, return to implementation and re-review.
6. If spec review passes, run quality review.
7. If quality review fails, return to implementation and re-review.
8. Only after both reviews pass may the parent accept the task.

## Deterministic Outcomes

- `implementation`
- `implementation_evidence`
- `spec_review`
- `quality_review`
- `repair`
- `accept`
- `done`

