import test from "node:test";
import assert from "node:assert/strict";
import { mkdtempSync, readFileSync, writeFileSync } from "node:fs";
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

**Task Status:** ready
**Current Step:** none
**Spec Review Status:** pending
**Quality Review Status:** pending
**Assigned Agent:** unassigned

- [ ] Step 1: First action

### Task T2: Second Task

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
