import test from "node:test";
import assert from "node:assert/strict";
import { existsSync, mkdirSync, mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { CategoryRegistry } from "../src/services/category-registry.ts";
import { RuntimeStore } from "../src/services/runtime-store.ts";
import { createTools } from "../src/tools/register-tools.ts";

function createFixtureCategories(file: string) {
  writeFileSync(file, `
[backend-impl]
intent = "implementation"
preferred_role = "backend-developer"
allowed_roles = ["backend-developer"]
write_policy = "lease-required"
requires_plan = true
requires_spec_review = true
requires_quality_review = true
parallelism = "write-scope"
delegation_preference = "subagent-required"
reuse_policy = "same_task_same_role_same_scope"
completion_contract = ["task_accepted"]

[review]
intent = "review"
preferred_role = "harness-evaluator"
allowed_roles = ["harness-evaluator"]
write_policy = "read-only"
requires_plan = true
requires_spec_review = false
requires_quality_review = false
parallelism = "parallel"
delegation_preference = "subagent-required"
reuse_policy = "no_reuse"
completion_contract = ["review_recorded"]
`.trim());
}

function createPlanFixture(file: string) {
  writeFileSync(file, `# Fixture Plan

## Execution Status

- Current wave: Wave 1
- Active task: P2-T2
- Blockers: None
- Last review result: Not started

## TODO List

- [ ] P2-T2. Lease Task
- [ ] P2-T3. Review Task

### Task P2-T2: Lease Task

**Category:** backend-impl
**Owner Role:** backend-developer
**Task Status:** ready
**Current Step:** none
**Spec Review Status:** pending
**Quality Review Status:** pending
**Assigned Agent:** unassigned

- [ ] Step 1: Do implementation

### Task P2-T3: Review Task

**Category:** review
**Owner Role:** harness-evaluator
**Task Status:** planned
**Current Step:** none
**Spec Review Status:** pending
**Quality Review Status:** pending
**Assigned Agent:** unassigned

- [ ] Step 1: Review implementation

## Final Acceptance

- [ ] done
`);
}

function createPhase3PlanFixture(file: string) {
  writeFileSync(file, `# Phase 3 Plan

## Execution Status

- Current wave: Wave 2
- Active task: P3-T2
- Blockers: None
- Last review result: Not started

## TODO List

- [ ] P3-T2. Question Gate
- [ ] P3-T3. Completion Assessment

### Task P3-T2: Question Gate

**Category:** backend-impl
**Owner Role:** backend-developer
**Task Status:** ready
**Current Step:** none
**Spec Review Status:** pending
**Quality Review Status:** pending
**Assigned Agent:** unassigned

- [ ] Step 1: Add the tool

### Task P3-T3: Completion Assessment

**Category:** backend-impl
**Owner Role:** backend-developer
**Task Status:** impl_done
**Current Step:** none
**Spec Review Status:** pending
**Quality Review Status:** pending
**Assigned Agent:** agent-impl

- [ ] Step 1: Finish implementation

## Final Acceptance

- [ ] all done
`);
}

function createMultiStepPlanFixture(file: string) {
  writeFileSync(file, `# Multi Step Plan

## Execution Status

- Current wave: Wave 1
- Active task: P2-T2
- Blockers: None
- Last review result: Not started

## TODO List

- [ ] P2-T2. Lease Task

### Task P2-T2: Lease Task

**Category:** backend-impl
**Owner Role:** backend-developer
**Task Status:** ready
**Current Step:** none
**Spec Review Status:** pending
**Quality Review Status:** pending
**Assigned Agent:** unassigned

- [ ] Step 1: First implementation action
- [ ] Step 2: Second implementation action

## Final Acceptance

- [ ] done
`);
}

function createReviewInProgressPlanFixture(file: string) {
  writeFileSync(file, `# Review Plan

## Execution Status

- Current wave: Wave Review
- Active task: R1
- Blockers: None
- Last review result: Not started

## TODO List

- [x] B1. Completed Build Task
- [ ] R1. Quality Review Task

### Task B1: Completed Build Task

**Category:** backend-impl
**Owner Role:** backend-developer
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** agent-build

- [x] Step 1: Build implementation

### Task R1: Quality Review Task

**Category:** review
**Owner Role:** harness-evaluator
**Task Status:** running_quality_review
**Current Step:** Step 1
**Spec Review Status:** pending
**Quality Review Status:** pending
**Assigned Agent:** agent-review

- [ ] Step 1: Review implementation

## Final Acceptance

- [ ] final review done
`);
}

function createCodexTodoMirrorPlanFixture(file: string) {
  writeFileSync(file, `# Codex Todo Mirror Plan

## Execution Status

- Current wave: Wave Mirror
- Active task: A1
- Blockers: None
- Last review result: Not started

## TODO List

- [x] C1. Completed Setup Task
- [ ] A1. Active Implementation Task
- [ ] P1. Pending Review Task

### Task C1: Completed Setup Task

**Category:** backend-impl
**Owner Role:** backend-developer
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** agent-setup

- [x] Step 1: Finish setup

### Task A1: Active Implementation Task

**Category:** backend-impl
**Owner Role:** backend-developer
**Task Status:** running_impl
**Current Step:** Step 1
**Spec Review Status:** pending
**Quality Review Status:** pending
**Assigned Agent:** agent-impl

- [ ] Step 1: Implement export tool

### Task P1: Pending Review Task

**Category:** review
**Owner Role:** harness-evaluator
**Task Status:** planned
**Current Step:** none
**Spec Review Status:** pending
**Quality Review Status:** pending
**Assigned Agent:** unassigned

- [ ] Step 1: Review the exported payload

## Final Acceptance

- [ ] final acceptance
`);
}

function createFinalAcceptancePlanFixture(file: string) {
  writeFileSync(file, `# Final Acceptance Plan

## Execution Status

- Current wave: Wave Finish
- Active task: none
- Blockers: None
- Last review result: quality pass

## TODO List

- [x] D1. Completed Task

### Task D1: Completed Task

**Category:** backend-impl
**Owner Role:** backend-developer
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** agent-impl

- [x] Step 1: Do implementation

## Final Acceptance

- [ ] final acceptance
`);
}

function createCompletedPlanFixture(file: string) {
  writeFileSync(file, `# Completed Plan

## Execution Status

- Current wave: Wave Finish
- Active task: none
- Blockers: None
- Last review result: quality pass

## TODO List

- [x] D1. Completed Task

### Task D1: Completed Task

**Category:** backend-impl
**Owner Role:** backend-developer
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** agent-impl

- [x] Step 1: Do implementation

## Final Acceptance

- [x] final acceptance
`);
}

function createCompletedActivePlanFixture(file: string) {
  writeFileSync(file, `# Completed Active Plan

## Execution Status

- Current wave: Wave Finish
- Active task: none
- Blockers: None
- Last review result: quality pass

## TODO List

- [x] D1. Completed Task

### Task D1: Completed Task

**Category:** backend-impl
**Owner Role:** backend-developer
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** agent-impl

- [x] Step 1: Do implementation

## Final Acceptance

- [x] final acceptance
`);
}

function createArchivePaths(name: string) {
  const dir = mkdtempSync(join(tmpdir(), "codex-orchestrator-tools-"));
  const activeDir = join(dir, "docs", "plans", "active");
  const completedDir = join(dir, "docs", "plans", "completed");
  mkdirSync(activeDir, { recursive: true });
  mkdirSync(completedDir, { recursive: true });
  return {
    dir,
    activeFile: join(activeDir, name),
    completedFile: join(completedDir, name),
  };
}

function getTool(name: string, tools: ReturnType<typeof createTools>) {
  const tool = tools.find((entry) => entry.name === name);
  assert.ok(tool, `tool not found: ${name}`);
  return tool;
}

test("begin_task rejects lease-required implementation without a lease", async () => {
  const dir = mkdtempSync(join(tmpdir(), "codex-orchestrator-tools-"));
  const categoriesFile = join(dir, "categories.toml");
  const planFile = join(dir, "plan.md");
  createFixtureCategories(categoriesFile);
  createPlanFixture(planFile);

  const registry = CategoryRegistry.fromToml(categoriesFile);
  const store = new RuntimeStore(join(dir, "state.db"));
  const tools = createTools({ categories: registry, runtimeStore: store });
  const beginTask = getTool("orchestrator_begin_task", tools);

  await assert.rejects(
    () => beginTask.handler({
      planPath: planFile,
      taskId: "P2-T2",
      categoryId: "backend-impl",
      role: "backend-developer",
    }),
    /write lease required/,
  );
});

test("begin_task seeds the first unchecked step and returns step guidance", async () => {
  const dir = mkdtempSync(join(tmpdir(), "codex-orchestrator-tools-"));
  const categoriesFile = join(dir, "categories.toml");
  const planFile = join(dir, "multi-step-plan.md");
  createFixtureCategories(categoriesFile);
  createMultiStepPlanFixture(planFile);

  const registry = CategoryRegistry.fromToml(categoriesFile);
  const store = new RuntimeStore(join(dir, "state.db"));
  store.upsertPlanState({
    planId: "multi-step-plan",
    planPath: planFile,
    activeTaskId: "P2-T2",
    currentWave: "Wave 1",
  });
  store.acquireWriteLease({
    planId: "multi-step-plan",
    taskId: "P2-T2",
    holderAgentId: "agent-1",
    scope: ["src/**"],
  });
  const tools = createTools({ categories: registry, runtimeStore: store });
  const beginTask = getTool("orchestrator_begin_task", tools);
  const readPlanState = getTool("orchestrator_read_plan_state", tools);

  const response = await beginTask.handler({
    planPath: planFile,
    taskId: "P2-T2",
    categoryId: "backend-impl",
    role: "backend-developer",
    assignedAgent: "agent-1",
  });
  const payload = response.structuredContent as {
    current_step_label: string;
    next_step_label: string;
    next_step_text: string;
    remaining_step_count: number;
    step_sync_status: string;
    step_sync_action: string;
  };

  assert.equal(payload.current_step_label, "Step 1");
  assert.equal(payload.next_step_label, "Step 1");
  assert.equal(payload.next_step_text, "First implementation action");
  assert.equal(payload.remaining_step_count, 2);
  assert.equal(payload.step_sync_status, "step_in_progress");
  assert.equal(payload.step_sync_action, "continue_current_step");

  const planState = (await readPlanState.handler({ planPath: planFile })).structuredContent as {
    tasks: Array<{ id: string; currentStep: string }>;
  };
  assert.equal(planState.tasks[0]?.currentStep, "Step 1");
});

test("complete_step auto-advances to the next unchecked step", async () => {
  const dir = mkdtempSync(join(tmpdir(), "codex-orchestrator-tools-"));
  const categoriesFile = join(dir, "categories.toml");
  const planFile = join(dir, "multi-step-plan.md");
  createFixtureCategories(categoriesFile);
  createMultiStepPlanFixture(planFile);

  const registry = CategoryRegistry.fromToml(categoriesFile);
  const store = new RuntimeStore(join(dir, "state.db"));
  store.upsertPlanState({
    planId: "multi-step-plan",
    planPath: planFile,
    activeTaskId: "P2-T2",
    currentWave: "Wave 1",
  });
  store.acquireWriteLease({
    planId: "multi-step-plan",
    taskId: "P2-T2",
    holderAgentId: "agent-1",
    scope: ["src/**"],
  });
  const tools = createTools({ categories: registry, runtimeStore: store });
  const beginTask = getTool("orchestrator_begin_task", tools);
  const completeStep = getTool("orchestrator_complete_step", tools);
  const readPlanState = getTool("orchestrator_read_plan_state", tools);

  await beginTask.handler({
    planPath: planFile,
    taskId: "P2-T2",
    categoryId: "backend-impl",
    role: "backend-developer",
    assignedAgent: "agent-1",
  });

  const response = await completeStep.handler({
    planPath: planFile,
    taskId: "P2-T2",
    stepLabel: "Step 1",
    evidenceSummary: "first step done",
  });
  const payload = response.structuredContent as {
    next_step_label: string;
    next_step_text: string;
    current_step_label: string;
    remaining_step_count: number;
    auto_advanced: boolean;
    step_sync_status: string;
    step_sync_action: string;
  };

  assert.equal(payload.auto_advanced, true);
  assert.equal(payload.current_step_label, "Step 2");
  assert.equal(payload.next_step_label, "Step 2");
  assert.equal(payload.next_step_text, "Second implementation action");
  assert.equal(payload.remaining_step_count, 1);
  assert.equal(payload.step_sync_status, "step_in_progress");
  assert.equal(payload.step_sync_action, "continue_current_step");

  const planState = (await readPlanState.handler({ planPath: planFile })).structuredContent as {
    tasks: Array<{ id: string; currentStep: string; steps: Array<{ label: string; checked: boolean }> }>;
  };
  assert.equal(planState.tasks[0]?.currentStep, "Step 2");
  assert.equal(planState.tasks[0]?.steps[0]?.checked, true);
});

test("next_action asks for a write lease before dispatching lease-required work", async () => {
  const dir = mkdtempSync(join(tmpdir(), "codex-orchestrator-tools-"));
  const categoriesFile = join(dir, "categories.toml");
  const planFile = join(dir, "plan.md");
  createFixtureCategories(categoriesFile);
  createPlanFixture(planFile);

  const registry = CategoryRegistry.fromToml(categoriesFile);
  const store = new RuntimeStore(join(dir, "state.db"));
  const tools = createTools({ categories: registry, runtimeStore: store });
  const nextAction = getTool("orchestrator_next_action", tools);

  const response = await nextAction.handler({ planPath: planFile });
  const payload = response.structuredContent as {
    action: string;
    task_id: string;
    requires_write_lease: boolean;
    requires_subagent: boolean;
    dispatch_role?: string;
    intervention_reason: string;
    dispatch_mode: string;
  };
  assert.equal(payload.task_id, "P2-T2");
  assert.equal(payload.action, "acquire_write_lease");
  assert.equal(payload.requires_write_lease, true);
  assert.equal(payload.requires_subagent, false);
  assert.equal(payload.dispatch_role, undefined);
  assert.match(payload.intervention_reason, /parent-owned control-plane/i);
  assert.equal(payload.dispatch_mode, "parent-local");
});

test("next_action exposes step guidance and detects missing current-step sync", async () => {
  const dir = mkdtempSync(join(tmpdir(), "codex-orchestrator-tools-"));
  const categoriesFile = join(dir, "categories.toml");
  const planFile = join(dir, "multi-step-plan.md");
  createFixtureCategories(categoriesFile);
  createMultiStepPlanFixture(planFile);

  const registry = CategoryRegistry.fromToml(categoriesFile);
  const store = new RuntimeStore(join(dir, "state.db"));
  store.acquireWriteLease({
    planId: "multi-step-plan",
    taskId: "P2-T2",
    holderAgentId: "agent-1",
    scope: ["src/**"],
  });
  store.upsertTaskState({
    planId: "multi-step-plan",
    taskId: "P2-T2",
    categoryId: "backend-impl",
    status: "running_impl",
    assignedRole: "backend-developer",
    agentId: "agent-1",
    activeStepLabel: null,
  });

  const tools = createTools({ categories: registry, runtimeStore: store });
  const nextAction = getTool("orchestrator_next_action", tools);
  const response = await nextAction.handler({ planPath: planFile });
  const payload = response.structuredContent as {
    action: string;
    step_sync_status: string;
    step_sync_action: string;
    next_step_label: string;
    next_step_text: string;
    remaining_step_count: number;
  };

  assert.equal(payload.action, "continue_same_agent");
  assert.equal(payload.step_sync_status, "needs_begin_step");
  assert.equal(payload.step_sync_action, "begin_next_step");
  assert.equal(payload.next_step_label, "Step 1");
  assert.equal(payload.next_step_text, "First implementation action");
  assert.equal(payload.remaining_step_count, 2);
});

test("export_codex_todo mirrors completed, in-progress, and pending work for native update_plan", async () => {
  const dir = mkdtempSync(join(tmpdir(), "codex-orchestrator-tools-"));
  const categoriesFile = join(dir, "categories.toml");
  const planFile = join(dir, "codex-todo-plan.md");
  createFixtureCategories(categoriesFile);
  createCodexTodoMirrorPlanFixture(planFile);

  const registry = CategoryRegistry.fromToml(categoriesFile);
  const store = new RuntimeStore(join(dir, "state.db"));
  store.upsertTaskState({
    planId: "codex-todo-plan",
    taskId: "A1",
    categoryId: "backend-impl",
    status: "running_impl",
    assignedRole: "backend-developer",
    agentId: "agent-impl",
    activeStepLabel: "Step 1",
  });

  const tools = createTools({ categories: registry, runtimeStore: store });
  const exportCodexTodo = getTool("orchestrator_export_codex_todo", tools);
  const response = await exportCodexTodo.handler({ planPath: planFile });
  const payload = response.structuredContent as {
    items: Array<{ step: string; status: string }>;
    active_task_id: string;
    current_step_label: string;
    current_step_text: string;
    step_sync_status: string;
  };

  assert.equal(payload.active_task_id, "A1");
  assert.equal(payload.current_step_label, "Step 1");
  assert.equal(payload.current_step_text, "Implement export tool");
  assert.equal(payload.step_sync_status, "step_in_progress");
  assert.deepEqual(payload.items.map((item) => item.status), ["completed", "in_progress", "pending"]);
  assert.match(payload.items[1]?.step ?? "", /A1\. Active Implementation Task/);
  assert.match(payload.items[1]?.step ?? "", /Step 1: Implement export tool/);
});

test("export_codex_todo mirrors final acceptance when it is the only remaining work", async () => {
  const dir = mkdtempSync(join(tmpdir(), "codex-orchestrator-tools-"));
  const categoriesFile = join(dir, "categories.toml");
  const planFile = join(dir, "final-acceptance-plan.md");
  createFixtureCategories(categoriesFile);
  createFinalAcceptancePlanFixture(planFile);

  const registry = CategoryRegistry.fromToml(categoriesFile);
  const store = new RuntimeStore(join(dir, "state.db"));
  const tools = createTools({ categories: registry, runtimeStore: store });
  const exportCodexTodo = getTool("orchestrator_export_codex_todo", tools);
  const response = await exportCodexTodo.handler({ planPath: planFile });
  const payload = response.structuredContent as {
    items: Array<{ step: string; status: string }>;
    open_acceptance_items: string[];
  };

  assert.equal(payload.items.length, 2);
  assert.equal(payload.items[0]?.status, "completed");
  assert.equal(payload.items[1]?.status, "in_progress");
  assert.match(payload.items[1]?.step ?? "", /Final acceptance/i);
  assert.ok(payload.open_acceptance_items.includes("final acceptance"));
});

test("resolve_category exposes default subagent bias", async () => {
  const dir = mkdtempSync(join(tmpdir(), "codex-orchestrator-tools-"));
  const categoriesFile = join(dir, "categories.toml");
  createFixtureCategories(categoriesFile);

  const registry = CategoryRegistry.fromToml(categoriesFile);
  const store = new RuntimeStore(join(dir, "state.db"));
  const tools = createTools({ categories: registry, runtimeStore: store });
  const resolveCategory = getTool("orchestrator_resolve_category", tools);

  const response = await resolveCategory.handler({
    title: "Review current runtime",
    description: "Review and verify orchestration behavior",
  });
  const payload = response.structuredContent as {
    category_id: string;
    preferred_role: string;
    delegation_preference: string;
    requires_subagent_default: boolean;
  };
  assert.equal(payload.category_id, "review");
  assert.equal(payload.preferred_role, "harness-evaluator");
  assert.equal(payload.delegation_preference, "subagent-required");
  assert.equal(payload.requires_subagent_default, true);
});

test("next_action recommends returning to implementer after review failure", async () => {
  const dir = mkdtempSync(join(tmpdir(), "codex-orchestrator-tools-"));
  const categoriesFile = join(dir, "categories.toml");
  const planFile = join(dir, "plan.md");
  createFixtureCategories(categoriesFile);
  createPlanFixture(planFile);

  const registry = CategoryRegistry.fromToml(categoriesFile);
  const store = new RuntimeStore(join(dir, "state.db"));
  store.acquireWriteLease({
    planId: "plan",
    taskId: "P2-T2",
    holderAgentId: "agent-1",
    scope: ["src/**"],
  });
  store.upsertTaskState({
    planId: "plan",
    taskId: "P2-T2",
    categoryId: "backend-impl",
    status: "quality_failed",
    assignedRole: "backend-developer",
    agentId: "agent-1",
  });

  const tools = createTools({ categories: registry, runtimeStore: store });
  const watchdog = getTool("orchestrator_watchdog_tick", tools);

  const response = await watchdog.handler({ planId: "plan", olderThanMs: -1 });
  const payload = response.structuredContent as {
    stalled_tasks: Array<{ task_id: string; suggested_action: string }>;
  };
  assert.equal(payload.stalled_tasks[0]?.task_id, "P2-T2");
  assert.equal(payload.stalled_tasks[0]?.suggested_action, "return_to_implementer");
});

test("watchdog prefers step-sync repair when a running task has open steps but no current step", async () => {
  const dir = mkdtempSync(join(tmpdir(), "codex-orchestrator-tools-"));
  const categoriesFile = join(dir, "categories.toml");
  const planFile = join(dir, "multi-step-plan.md");
  createFixtureCategories(categoriesFile);
  createMultiStepPlanFixture(planFile);

  const registry = CategoryRegistry.fromToml(categoriesFile);
  const store = new RuntimeStore(join(dir, "state.db"));
  store.upsertPlanState({
    planId: "multi-step-plan",
    planPath: planFile,
    activeTaskId: "P2-T2",
    currentWave: "Wave 1",
  });
  store.acquireWriteLease({
    planId: "multi-step-plan",
    taskId: "P2-T2",
    holderAgentId: "agent-1",
    scope: ["src/**"],
  });
  store.upsertTaskState({
    planId: "multi-step-plan",
    taskId: "P2-T2",
    categoryId: "backend-impl",
    status: "running_impl",
    assignedRole: "backend-developer",
    agentId: "agent-1",
    activeStepLabel: null,
  });

  const tools = createTools({ categories: registry, runtimeStore: store });
  const watchdog = getTool("orchestrator_watchdog_tick", tools);
  const response = await watchdog.handler({ planId: "multi-step-plan", olderThanMs: -1 });
  const payload = response.structuredContent as {
    stalled_tasks: Array<{ task_id: string; suggested_action: string }>;
  };

  assert.equal(payload.stalled_tasks[0]?.task_id, "P2-T2");
  assert.equal(payload.stalled_tasks[0]?.suggested_action, "repair_step_sync");
});

test("question gate rejects optional expansion questions by default", async () => {
  const dir = mkdtempSync(join(tmpdir(), "codex-orchestrator-tools-"));
  const categoriesFile = join(dir, "categories.toml");
  createFixtureCategories(categoriesFile);
  const registry = CategoryRegistry.fromToml(categoriesFile);
  const store = new RuntimeStore(join(dir, "state.db"));
  const tools = createTools({ categories: registry, runtimeStore: store });
  const questionGate = getTool("orchestrator_question_gate", tools);

  const response = await questionGate.handler({
    questionCategory: "optional_expansion",
    userExplicitlyRequested: false,
    reason: "Could ask whether to add extra analytics support",
  });
  const payload = response.structuredContent as {
    ask_user: boolean;
    allowed_to_expand: boolean;
  };
  assert.equal(payload.ask_user, false);
  assert.equal(payload.allowed_to_expand, false);
});

test("completion assessment blocks acceptance when steps and evidence are missing", async () => {
  const dir = mkdtempSync(join(tmpdir(), "codex-orchestrator-tools-"));
  const categoriesFile = join(dir, "categories.toml");
  const planFile = join(dir, "plan.md");
  createFixtureCategories(categoriesFile);
  createPhase3PlanFixture(planFile);

  const registry = CategoryRegistry.fromToml(categoriesFile);
  const store = new RuntimeStore(join(dir, "state.db"));
  store.upsertTaskState({
    planId: "plan",
    taskId: "P3-T3",
    categoryId: "backend-impl",
    status: "impl_done",
    assignedRole: "backend-developer",
    agentId: "agent-impl",
  });

  const tools = createTools({ categories: registry, runtimeStore: store });
  const assess = getTool("orchestrator_assess_subagent_completion", tools);
  const response = await assess.handler({
    planPath: planFile,
    taskId: "P3-T3",
  });
  const payload = response.structuredContent as {
    implementation_complete: boolean;
    missing_steps: string[];
    missing_evidence: boolean;
    can_accept: boolean;
    next_required_stage: string;
  };
  assert.equal(payload.implementation_complete, false);
  assert.equal(payload.missing_evidence, true);
  assert.equal(payload.can_accept, false);
  assert.equal(payload.next_required_stage, "implementation");
  assert.equal(payload.missing_steps[0], "Step 1");
});

test("completion guard fails closed when final acceptance is not complete", async () => {
  const dir = mkdtempSync(join(tmpdir(), "codex-orchestrator-tools-"));
  const categoriesFile = join(dir, "categories.toml");
  const planFile = join(dir, "plan.md");
  createFixtureCategories(categoriesFile);
  createPhase3PlanFixture(planFile);

  const registry = CategoryRegistry.fromToml(categoriesFile);
  const store = new RuntimeStore(join(dir, "state.db"));
  const tools = createTools({ categories: registry, runtimeStore: store });
  const guard = getTool("orchestrator_completion_guard", tools);
  const response = await guard.handler({ planPath: planFile });
  const payload = response.structuredContent as {
    can_finish: boolean;
    open_tasks: string[];
    open_acceptance_items: string[];
  };
  assert.equal(payload.can_finish, false);
  assert.ok(payload.open_tasks.includes("P3-T2"));
  assert.ok(payload.open_acceptance_items.includes("all done"));
});

test("next_action continues the same review agent for in-progress review work", async () => {
  const dir = mkdtempSync(join(tmpdir(), "codex-orchestrator-tools-"));
  const categoriesFile = join(dir, "categories.toml");
  const planFile = join(dir, "review-plan.md");
  createFixtureCategories(categoriesFile);
  createReviewInProgressPlanFixture(planFile);

  const registry = CategoryRegistry.fromToml(categoriesFile);
  const store = new RuntimeStore(join(dir, "state.db"));
  store.upsertTaskState({
    planId: "review-plan",
    taskId: "R1",
    categoryId: "review",
    status: "running_quality_review",
    assignedRole: "harness-evaluator",
    agentId: "agent-review",
  });

  const tools = createTools({ categories: registry, runtimeStore: store });
  const nextAction = getTool("orchestrator_next_action", tools);
  const response = await nextAction.handler({ planPath: planFile });
  const payload = response.structuredContent as {
    action: string;
    required_role: string;
    requires_subagent: boolean;
    dispatch_role: string;
  };

  assert.equal(payload.action, "continue_same_agent");
  assert.equal(payload.required_role, "harness-evaluator");
  assert.equal(payload.requires_subagent, true);
  assert.equal(payload.dispatch_role, "harness-evaluator");
});

test("next_action returns parent-local delegation metadata for final acceptance", async () => {
  const dir = mkdtempSync(join(tmpdir(), "codex-orchestrator-tools-"));
  const categoriesFile = join(dir, "categories.toml");
  const planFile = join(dir, "final-acceptance-plan.md");
  createFixtureCategories(categoriesFile);
  createFinalAcceptancePlanFixture(planFile);

  const registry = CategoryRegistry.fromToml(categoriesFile);
  const store = new RuntimeStore(join(dir, "state.db"));
  const tools = createTools({ categories: registry, runtimeStore: store });
  const nextAction = getTool("orchestrator_next_action", tools);
  const response = await nextAction.handler({ planPath: planFile });
  const payload = response.structuredContent as {
    action: string;
    requires_subagent: boolean;
    dispatch_role?: string;
    intervention_reason: string;
    dispatch_mode: string;
  };

  assert.equal(payload.action, "complete_final_acceptance");
  assert.equal(payload.requires_subagent, false);
  assert.equal(payload.dispatch_role, undefined);
  assert.match(payload.intervention_reason, /parent-owned control-plane/i);
  assert.equal(payload.dispatch_mode, "parent-local");
});

test("next_action returns parent-local delegation metadata for complete plan", async () => {
  const dir = mkdtempSync(join(tmpdir(), "codex-orchestrator-tools-"));
  const categoriesFile = join(dir, "categories.toml");
  const planFile = join(dir, "completed-plan.md");
  createFixtureCategories(categoriesFile);
  createCompletedPlanFixture(planFile);

  const registry = CategoryRegistry.fromToml(categoriesFile);
  const store = new RuntimeStore(join(dir, "state.db"));
  const tools = createTools({ categories: registry, runtimeStore: store });
  const nextAction = getTool("orchestrator_next_action", tools);
  const response = await nextAction.handler({ planPath: planFile });
  const payload = response.structuredContent as {
    action: string;
    requires_subagent: boolean;
    dispatch_role?: string;
    intervention_reason: string;
    dispatch_mode: string;
  };

  assert.equal(payload.action, "complete_plan");
  assert.equal(payload.requires_subagent, false);
  assert.equal(payload.dispatch_role, undefined);
  assert.match(payload.intervention_reason, /parent-owned control-plane/i);
  assert.equal(payload.dispatch_mode, "parent-local");
});

test("read_plan_state archives a completed active plan before returning state", async () => {
  const { dir, activeFile, completedFile } = createArchivePaths("archived-by-read.md");
  const categoriesFile = join(dir, "categories.toml");
  createFixtureCategories(categoriesFile);
  createCompletedActivePlanFixture(activeFile);

  const registry = CategoryRegistry.fromToml(categoriesFile);
  const store = new RuntimeStore(join(dir, "state.db"));
  const tools = createTools({ categories: registry, runtimeStore: store });
  const readPlanState = getTool("orchestrator_read_plan_state", tools);

  const response = await readPlanState.handler({ planPath: activeFile });
  const payload = response.structuredContent as {
    tasks: Array<{ id: string }>;
  };

  assert.equal(payload.tasks[0]?.id, "D1");
  assert.equal(existsSync(activeFile), false);
  assert.equal(existsSync(completedFile), true);
});
