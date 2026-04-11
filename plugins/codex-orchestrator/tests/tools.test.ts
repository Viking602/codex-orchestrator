import test from "node:test";
import assert from "node:assert/strict";
import { mkdtempSync, writeFileSync } from "node:fs";
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
