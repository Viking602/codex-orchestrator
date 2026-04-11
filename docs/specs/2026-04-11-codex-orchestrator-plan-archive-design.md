# Codex Orchestrator Completed Plan Auto-Archive Design

## Context

The repository already distinguishes between `docs/plans/active/` and `docs/plans/completed/`, but the workflow stops at naming. In practice, completed implementation plans are still left under `active/`, routing docs keep linking to them as if they were still live, and `PlanDocument` always writes back to the original path even after a plan is fully done.

That creates three defects:

- finished plans stay mixed into the execution queue
- historical routing docs point at the wrong directory
- first-touch cleanup of stale plan placement still depends on manual intervention

The desired behavior is stricter: if an implementation plan under `docs/plans/active/` is fully complete, it should automatically move to `docs/plans/completed/` without waiting for a human to notice and clean it up.

## Goals

- Auto-archive completed implementation plans from `docs/plans/active/` to `docs/plans/completed/`.
- Repair historical completed plans in `active/` on first touch without asking the user.
- Preserve compatibility when callers still open the stale `active/` path after archival.
- Keep routing docs, plan references, and agent guidance aligned with the archive model.
- Add regression coverage for both new completion and legacy first-touch repair.

## Non-goals

- Archiving incomplete plans.
- Deleting completed plans instead of relocating them.
- Introducing a separate archival daemon or background watcher.
- Replacing the current filename-based `plan_id` model.

## Design

### 1. Completion Becomes A File-Movement Trigger

A plan under `docs/plans/active/` should be considered archiveable when:

- every top-level TODO item is checked
- every `Final Acceptance` item is checked, if that section exists

When the plan reaches that state, the file should move to the matching path under `docs/plans/completed/` in the same repository pass.

### 2. First-Touch Repair Is Mandatory

When repository code first opens an `active/` plan that is already complete, the workflow should repair that stale placement immediately by moving it to `completed/`. This is hygiene debt, not a user-choice surface.

Compatibility rule:

- if a caller still asks for the stale `active/` path after archival, the plan layer should transparently resolve it to the matching completed path when that archived file exists

### 3. PlanDocument Owns Path Reconciliation

`PlanDocument` should stop treating `planPath` as an immutable write target. Instead it should:

- resolve stale active paths to completed paths when the archived file already exists
- archive completed active plans on read
- archive completed active plans after writes that make the plan complete
- expose the current effective `planPath` after reconciliation

This keeps path repair close to the markdown truth surface instead of scattering it through callers.

### 4. Routing And Historical Docs Must Follow The Archive Model

The durable docs should describe the archive lifecycle accurately:

- `docs/index.md`
- `AGENTS.md`
- architecture notes that define plan synchronization and artifact roots
- bundled agent instructions that tell child roles where to read active versus historical plans

Historical completed-plan links should point at `docs/plans/completed/`, not `docs/plans/active/`.

### 5. Regression Coverage

Add tests that fail closed when:

- a completed plan under `docs/plans/active/` is read but not archived
- a write that completes the final acceptance state does not trigger archival
- stale callers cannot reopen an already archived plan through the old active path

## Success Criteria

- Completed plans no longer remain in `docs/plans/active/` after first touch, including legacy plans without a `Final Acceptance` section.
- Newly completed active plans auto-move to `docs/plans/completed/`.
- `PlanDocument` resolves stale active paths to the archived file when needed.
- Routing docs and bundled-agent guidance describe `active/` as live work and `completed/` as historical reference.
- Tests cover archive-on-read and archive-on-completion behavior.
