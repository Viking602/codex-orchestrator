import test from "node:test";
import assert from "node:assert/strict";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { PlanDocument } from "../src/services/plan-document.ts";

const PLAN_FIXTURE = `# Test Plan

## Execution Status

- Current wave: Wave 1
- Active task: T1
- Blockers: None
- Last review result: Not started

## TODO List

- [ ] T1. Example Task

### Task T1: Example Task

**Category:** backend-impl
**Owner Role:** backend-developer
**Task Status:** ready
**Current Step:** none
**Spec Review Status:** pending
**Quality Review Status:** pending
**Assigned Agent:** unassigned

- [ ] Step 1: First action
- [ ] Step 2: Second action

## Final Acceptance

- [ ] done
`;

const COMPLETED_ACTIVE_PLAN_FIXTURE = `# Completed Active Plan

## Execution Status

- Current wave: Wave Finish
- Active task: none
- Blockers: None
- Last review result: quality pass

## TODO List

- [x] T1. Example Task

### Task T1: Example Task

**Category:** backend-impl
**Owner Role:** backend-developer
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** agent-1

- [x] Step 1: First action
- [x] Step 2: Second action

## Final Acceptance

- [x] done
`;

const READY_TO_ARCHIVE_PLAN_FIXTURE = `# Ready To Archive Plan

## Execution Status

- Current wave: Wave Finish
- Active task: none
- Blockers: None
- Last review result: quality pass

## TODO List

- [x] T1. Example Task

### Task T1: Example Task

**Category:** backend-impl
**Owner Role:** backend-developer
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** agent-1

- [x] Step 1: First action
- [x] Step 2: Second action

## Final Acceptance

- [ ] done
`;

const LEGACY_COMPLETED_ACTIVE_PLAN_FIXTURE = `# Legacy Completed Active Plan

## Execution Status

- Current wave: Wave Finish
- Active task: none
- Blockers: None
- Last review result: quality pass

## TODO List

- [x] T1. Example Task

### Task T1: Example Task

**Category:** backend-impl
**Owner Role:** backend-developer
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** agent-1

- [x] Step 1: First action
`;

function createArchiveFixture(name: string) {
  const dir = mkdtempSync(join(tmpdir(), "codex-orchestrator-plan-"));
  const activeDir = join(dir, "docs", "plans", "active");
  const completedDir = join(dir, "docs", "plans", "completed");
  mkdirSync(activeDir, { recursive: true });
  mkdirSync(completedDir, { recursive: true });
  return {
    activeFile: join(activeDir, name),
    completedFile: join(completedDir, name),
  };
}

test("PlanDocument marks steps and top-level todos", () => {
  const dir = mkdtempSync(join(tmpdir(), "codex-orchestrator-plan-"));
  const file = join(dir, "plan.md");
  writeFileSync(file, PLAN_FIXTURE);
  const plan = new PlanDocument(file);

  plan.updateExecutionStatus({ activeTask: "T1", currentWave: "Wave 2" });
  plan.updateTaskMetadata("T1", { taskStatus: "running_impl", currentStep: "Step 1", assignedAgent: "agent-1" });
  plan.markStep("T1", "Step 1", true);
  plan.markTopLevelTodo("T1", true);

  const content = readFileSync(file, "utf8");
  assert.match(content, /- Current wave: Wave 2/);
  assert.match(content, /\*\*Task Status:\*\* running_impl/);
  assert.match(content, /\*\*Assigned Agent:\*\* agent-1/);
  assert.match(content, /- \[x\] Step 1: First action/);
  assert.match(content, /- \[x\] T1\. Example Task/);
});

test("PlanDocument parses tasks and completion state", () => {
  const dir = mkdtempSync(join(tmpdir(), "codex-orchestrator-plan-"));
  const file = join(dir, "plan.md");
  writeFileSync(file, PLAN_FIXTURE);
  const plan = new PlanDocument(file);
  const state = plan.readPlanState();
  assert.equal(state.tasks.length, 1);
  assert.equal(state.tasks[0]?.id, "T1");
  assert.equal(state.tasks[0]?.steps.length, 2);
  assert.equal(plan.allStepsCompleted("T1"), false);
});

test("PlanDocument parses adjacent task blocks without skipping", () => {
  const dir = mkdtempSync(join(tmpdir(), "codex-orchestrator-plan-"));
  const file = join(dir, "plan.md");
  writeFileSync(file, `# Test Plan

## Execution Status

- Current wave: Wave 1
- Active task: T1
- Blockers: None
- Last review result: Not started

## TODO List

- [ ] T1. First Task
- [ ] T2. Second Task

### Task T1: First Task

**Category:** backend-impl
**Owner Role:** backend-developer
**Task Status:** ready
**Current Step:** none
**Spec Review Status:** pending
**Quality Review Status:** pending
**Assigned Agent:** unassigned

- [ ] Step 1: First action

### Task T2: Second Task

**Category:** review
**Owner Role:** harness-evaluator
**Task Status:** ready
**Current Step:** none
**Spec Review Status:** pending
**Quality Review Status:** pending
**Assigned Agent:** unassigned

- [ ] Step 1: Second action

## Final Acceptance

- [ ] done
`);
  const plan = new PlanDocument(file);
  const state = plan.readPlanState();
  assert.deepEqual(
    state.tasks.map((task) => task.id),
    ["T1", "T2"],
  );
});

test("PlanDocument archives completed active plans on read and resolves stale active paths", () => {
  const { activeFile, completedFile } = createArchiveFixture("completed-plan.md");
  writeFileSync(activeFile, COMPLETED_ACTIVE_PLAN_FIXTURE);

  const plan = new PlanDocument(activeFile);
  const state = plan.readPlanState();

  assert.equal(plan.planPath, completedFile);
  assert.equal(state.tasks[0]?.id, "T1");
  assert.equal(existsSync(activeFile), false);
  assert.equal(existsSync(completedFile), true);

  const reopened = new PlanDocument(activeFile);
  assert.equal(reopened.planPath, completedFile);
  assert.equal(reopened.readPlanState().tasks[0]?.id, "T1");
});

test("PlanDocument archives completed relative active plan paths", () => {
  const dir = mkdtempSync(join(tmpdir(), "codex-orchestrator-plan-"));
  const relativeActiveFile = "docs/plans/active/relative-completed-plan.md";
  const relativeCompletedFile = "docs/plans/completed/relative-completed-plan.md";
  mkdirSync(join(dir, "docs", "plans", "active"), { recursive: true });
  mkdirSync(join(dir, "docs", "plans", "completed"), { recursive: true });
  writeFileSync(join(dir, relativeActiveFile), COMPLETED_ACTIVE_PLAN_FIXTURE);

  const originalCwd = process.cwd();
  process.chdir(dir);
  try {
    const plan = new PlanDocument(relativeActiveFile);
    const state = plan.readPlanState();

    assert.equal(state.tasks[0]?.id, "T1");
    assert.equal(plan.planPath, relativeCompletedFile);
    assert.equal(existsSync(relativeActiveFile), false);
    assert.equal(existsSync(relativeCompletedFile), true);
  } finally {
    process.chdir(originalCwd);
  }
});

test("PlanDocument archives active plans after final acceptance becomes complete", () => {
  const { activeFile, completedFile } = createArchiveFixture("ready-to-archive-plan.md");
  writeFileSync(activeFile, READY_TO_ARCHIVE_PLAN_FIXTURE);

  const plan = new PlanDocument(activeFile);
  plan.markFinalAcceptance("done", true);

  assert.equal(plan.planPath, completedFile);
  assert.equal(existsSync(activeFile), false);
  assert.equal(existsSync(completedFile), true);
  assert.match(readFileSync(completedFile, "utf8"), /- \[x\] done/);
});

test("PlanDocument archives completed active plans with CRLF line endings", () => {
  const { activeFile, completedFile } = createArchiveFixture("completed-plan-crlf.md");
  writeFileSync(activeFile, COMPLETED_ACTIVE_PLAN_FIXTURE.replace(/\n/g, "\r\n"));

  const plan = new PlanDocument(activeFile);
  const state = plan.readPlanState();

  assert.equal(state.tasks[0]?.id, "T1");
  assert.equal(plan.planPath, completedFile);
  assert.equal(existsSync(activeFile), false);
  assert.equal(existsSync(completedFile), true);
});

test("PlanDocument archives legacy completed active plans without final acceptance", () => {
  const { activeFile, completedFile } = createArchiveFixture("legacy-completed-plan.md");
  writeFileSync(activeFile, LEGACY_COMPLETED_ACTIVE_PLAN_FIXTURE);

  const plan = new PlanDocument(activeFile);
  const state = plan.readPlanState();

  assert.equal(state.tasks[0]?.id, "T1");
  assert.equal(plan.planPath, completedFile);
  assert.equal(existsSync(activeFile), false);
  assert.equal(existsSync(completedFile), true);
});
