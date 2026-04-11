import { basename } from "node:path";
import { CategoryRegistry } from "../services/category-registry.ts";
import { checkDocDrift } from "../services/doc-drift.ts";
import { PlanDocument } from "../services/plan-document.ts";
import { RuntimeStore } from "../services/runtime-store.ts";
import type { ReviewStatus, TaskStatus } from "../types.ts";
import type { CategoryDefinition, PlanTask, TaskStateRecord } from "../types.ts";

export interface ToolDefinition {
  name: string;
  description: string;
  inputSchema: Record<string, unknown>;
  handler: (args: Record<string, unknown>) => Promise<{
    content: Array<{ type: "text"; text: string }>;
    structuredContent?: Record<string, unknown>;
    isError?: boolean;
  }>;
}

type ToolDeps = {
  categories: CategoryRegistry;
  runtimeStore: RuntimeStore;
};

function asText(data: unknown): string {
  return JSON.stringify(data, null, 2);
}

function result<T extends object>(payload: T) {
  return {
    content: [{ type: "text" as const, text: asText(payload) }],
    structuredContent: payload,
  };
}

function planIdFromPath(planPath: string): string {
  return basename(planPath, ".md");
}

function syncStoredPlanPath(runtimeStore: RuntimeStore, planId: string, planPath: string): void {
  const existing = runtimeStore.getPlanState(planId);
  if (!existing || existing.planPath === planPath) return;
  runtimeStore.upsertPlanState({
    planId,
    planPath,
    specPath: existing.specPath,
    currentWave: existing.currentWave,
    activeTaskId: existing.activeTaskId,
    lastReviewResult: existing.lastReviewResult,
  });
}

function requireString(args: Record<string, unknown>, key: string): string {
  const value = args[key];
  if (typeof value !== "string" || value.trim() === "") {
    throw new Error(`Expected string argument: ${key}`);
  }
  return value;
}

function optionalString(args: Record<string, unknown>, key: string): string | undefined {
  const value = args[key];
  return typeof value === "string" ? value : undefined;
}

function optionalNumber(args: Record<string, unknown>, key: string): number | undefined {
  const value = args[key];
  return typeof value === "number" ? value : undefined;
}

function requireStringArray(args: Record<string, unknown>, key: string): string[] {
  const value = args[key];
  if (!Array.isArray(value) || value.some((entry) => typeof entry !== "string")) {
    throw new Error(`Expected string[] argument: ${key}`);
  }
  return value as string[];
}

type StepSyncStatus = "all_steps_complete" | "step_in_progress" | "needs_begin_step" | "stale_current_step";
type StepSyncAction = "none" | "continue_current_step" | "begin_next_step" | "repair_current_step";
type CodexTodoStatus = "completed" | "in_progress" | "pending";

type StepProgressState = {
  currentStepLabel?: string;
  currentStepText?: string;
  nextStepLabel?: string;
  nextStepText?: string;
  remainingStepCount: number;
  stepSyncStatus: StepSyncStatus;
  stepSyncAction: StepSyncAction;
};

function emptyStepProgress(): StepProgressState {
  return {
    remainingStepCount: 0,
    stepSyncStatus: "all_steps_complete",
    stepSyncAction: "none",
  };
}

function resolveCodexTodoActiveTask(planTasks: PlanTask[], activeTaskId: string | null | undefined): PlanTask | undefined {
  const openTasks = planTasks.filter((task) => !task.todoChecked);
  if (openTasks.length === 0) return undefined;
  const declaredActiveTask = activeTaskId && activeTaskId !== "none"
    ? openTasks.find((task) => task.id === activeTaskId)
    : undefined;
  return declaredActiveTask ?? openTasks[0];
}

function formatCodexTodoStep(
  planTask: PlanTask,
  status: CodexTodoStatus,
  stepProgress?: StepProgressState,
): string {
  const base = `${planTask.id}. ${planTask.title}`;
  if (status !== "in_progress" || !stepProgress) return base;
  const stepLabel = stepProgress.currentStepLabel ?? stepProgress.nextStepLabel;
  const stepText = stepProgress.currentStepText ?? stepProgress.nextStepText;
  if (!stepLabel || !stepText) return base;
  return `${base} (${stepLabel}: ${stepText})`;
}

function formatFinalAcceptanceStep(openAcceptanceItems: string[]): string {
  if (openAcceptanceItems.length === 1) {
    return `Final acceptance (${openAcceptanceItems[0]})`;
  }
  return `Final acceptance (${openAcceptanceItems.length} items remaining)`;
}

function buildCodexTodoMirror(
  plan: PlanDocument,
  planId: string,
  runtimeStore: RuntimeStore,
): {
  items: Array<{ step: string; status: CodexTodoStatus }>;
  activeTaskId?: string;
  activeTaskTitle?: string;
  currentStepLabel?: string;
  currentStepText?: string;
  remainingStepCount: number;
  stepSyncStatus: StepSyncStatus;
  stepSyncAction: StepSyncAction;
  openAcceptanceItems: string[];
} {
  const planState = plan.readPlanState();
  const openAcceptanceItems = plan.uncheckedFinalAcceptanceItems();
  const activeTask = resolveCodexTodoActiveTask(planState.tasks, planState.executionStatus.activeTask);
  const activeTaskState = activeTask ? runtimeStore.getTaskState(planId, activeTask.id) : undefined;
  const stepProgress = activeTask ? buildStepProgress(activeTask, activeTaskState) : emptyStepProgress();
  const items = planState.tasks.map((task) => {
    let status: CodexTodoStatus = "pending";
    if (task.todoChecked) {
      status = "completed";
    } else if (activeTask && task.id === activeTask.id) {
      status = "in_progress";
    }
    return {
      step: formatCodexTodoStep(task, status, task.id === activeTask?.id ? stepProgress : undefined),
      status,
    };
  });

  if (!activeTask && openAcceptanceItems.length > 0) {
    items.push({
      step: formatFinalAcceptanceStep(openAcceptanceItems),
      status: "in_progress",
    });
  }

  return {
    items,
    activeTaskId: activeTask?.id,
    activeTaskTitle: activeTask?.title,
    currentStepLabel: stepProgress.currentStepLabel,
    currentStepText: stepProgress.currentStepText,
    remainingStepCount: stepProgress.remainingStepCount,
    stepSyncStatus: stepProgress.stepSyncStatus,
    stepSyncAction: stepProgress.stepSyncAction,
    openAcceptanceItems,
  };
}

function firstUncheckedStep(planTask: PlanTask) {
  return planTask.steps.find((step) => !step.checked);
}

function findStepByLabel(planTask: PlanTask, label: string | null | undefined) {
  if (!label || label === "none") return undefined;
  return planTask.steps.find((step) => step.label === label);
}

function buildStepProgress(planTask: PlanTask, taskState: TaskStateRecord | undefined): StepProgressState {
  const nextStep = firstUncheckedStep(planTask);
  const remainingStepCount = planTask.steps.filter((step) => !step.checked).length;
  if (!nextStep) {
    return {
      remainingStepCount: 0,
      stepSyncStatus: "all_steps_complete",
      stepSyncAction: "none",
    };
  }

  const declaredCurrentStepLabel = taskState?.activeStepLabel ?? (planTask.currentStep !== "none" ? planTask.currentStep : undefined);
  const currentStep = findStepByLabel(planTask, declaredCurrentStepLabel);
  const nextStepBase = {
    nextStepLabel: nextStep.label,
    nextStepText: nextStep.text,
    remainingStepCount,
  };

  if (!declaredCurrentStepLabel) {
    return {
      ...nextStepBase,
      stepSyncStatus: "needs_begin_step",
      stepSyncAction: "begin_next_step",
    };
  }

  if (!currentStep || currentStep.checked || currentStep.label !== nextStep.label) {
    return {
      ...nextStepBase,
      currentStepLabel: declaredCurrentStepLabel,
      stepSyncStatus: "stale_current_step",
      stepSyncAction: "repair_current_step",
    };
  }

  return {
    ...nextStepBase,
    currentStepLabel: currentStep.label,
    currentStepText: currentStep.text,
    stepSyncStatus: "step_in_progress",
    stepSyncAction: "continue_current_step",
  };
}

function stepProgressPayload(stepProgress: StepProgressState): Record<string, unknown> {
  return {
    current_step_label: stepProgress.currentStepLabel,
    current_step_text: stepProgress.currentStepText,
    next_step_label: stepProgress.nextStepLabel,
    next_step_text: stepProgress.nextStepText,
    remaining_step_count: stepProgress.remainingStepCount,
    step_sync_status: stepProgress.stepSyncStatus,
    step_sync_action: stepProgress.stepSyncAction,
  };
}

function statusUsesStepPointer(status: TaskStatus): boolean {
  return status === "running_impl" || status === "running_spec_review" || status === "running_quality_review";
}

function syncTaskCurrentStep(
  plan: PlanDocument,
  runtimeStore: RuntimeStore,
  planId: string,
  taskId: string,
  nextStepLabel: string | null,
): void {
  const current = runtimeStore.getTaskState(planId, taskId);
  if (!current) throw new Error(`Task state missing: ${taskId}`);
  runtimeStore.upsertTaskState({
    ...current,
    planId,
    taskId,
    categoryId: current.categoryId,
    status: current.status,
    activeStepLabel: nextStepLabel,
    assignedRole: current.assignedRole,
    agentId: current.agentId,
    writeLeaseId: current.writeLeaseId,
    specReviewStatus: current.specReviewStatus,
    qualityReviewStatus: current.qualityReviewStatus,
    retryCount: current.retryCount,
    blockerType: current.blockerType,
    blockerMessage: current.blockerMessage,
  });
  plan.updateTaskMetadata(taskId, { currentStep: nextStepLabel ?? "none" });
}

export function createTools(deps: ToolDeps): ToolDefinition[] {
  const { categories, runtimeStore } = deps;

  return [
    {
      name: "orchestrator_resolve_category",
      description: "Resolve a workflow category and allowed roles for a task.",
      inputSchema: {
        type: "object",
        properties: {
          title: { type: "string" },
          description: { type: "string" },
          explicitCategory: { type: "string" },
        },
        required: ["title", "description"],
      },
      handler: async (args) => {
        const resolution = categories.resolve({
          title: requireString(args, "title"),
          description: requireString(args, "description"),
          explicitCategory: optionalString(args, "explicitCategory"),
        });
        return result({
          category_id: resolution.categoryId,
          reason: resolution.reason,
          preferred_role: resolution.category.preferredRole,
          allowed_roles: resolution.category.allowedRoles,
          write_policy: resolution.category.writePolicy,
          requires_plan: resolution.category.requiresPlan,
          requires_spec_review: resolution.category.requiresSpecReview,
          requires_quality_review: resolution.category.requiresQualityReview,
          parallelism: resolution.category.parallelism,
          delegation_preference: resolution.category.delegationPreference,
          requires_subagent_default: resolution.category.delegationPreference !== "parent-only",
          reuse_policy: resolution.category.reusePolicy,
        });
      },
    },
    {
      name: "orchestrator_read_plan_state",
      description: "Read the current execution status and parsed tasks from an implementation plan.",
      inputSchema: {
        type: "object",
        properties: {
          planPath: { type: "string" },
        },
        required: ["planPath"],
      },
      handler: async (args) => {
        const plan = new PlanDocument(requireString(args, "planPath"));
        syncStoredPlanPath(runtimeStore, planIdFromPath(plan.planPath), plan.planPath);
        return result(plan.readPlanState());
      },
    },
    {
      name: "orchestrator_export_codex_todo",
      description: "Export the active implementation plan as mirror-ready items for Codex native update_plan.",
      inputSchema: {
        type: "object",
        properties: {
          planPath: { type: "string" },
        },
        required: ["planPath"],
      },
      handler: async (args) => {
        const requestedPlanPath = requireString(args, "planPath");
        const plan = new PlanDocument(requestedPlanPath);
        const planPath = plan.planPath;
        const planId = planIdFromPath(planPath);
        syncStoredPlanPath(runtimeStore, planId, planPath);
        const mirror = buildCodexTodoMirror(plan, planId, runtimeStore);
        return result({
          plan_id: planId,
          plan_path: planPath,
          items: mirror.items,
          active_task_id: mirror.activeTaskId,
          active_task_title: mirror.activeTaskTitle,
          current_step_label: mirror.currentStepLabel,
          current_step_text: mirror.currentStepText,
          remaining_step_count: mirror.remainingStepCount,
          step_sync_status: mirror.stepSyncStatus,
          step_sync_action: mirror.stepSyncAction,
          open_acceptance_items: mirror.openAcceptanceItems,
        });
      },
    },
    {
      name: "orchestrator_begin_task",
      description: "Mark a task as started in runtime state and in the active plan.",
      inputSchema: {
        type: "object",
        properties: {
          planPath: { type: "string" },
          taskId: { type: "string" },
          categoryId: { type: "string" },
          role: { type: "string" },
          taskStatus: { type: "string" },
          currentWave: { type: "string" },
          assignedAgent: { type: "string" },
        },
        required: ["planPath", "taskId", "categoryId", "role"],
      },
      handler: async (args) => {
        const requestedPlanPath = requireString(args, "planPath");
        const taskId = requireString(args, "taskId");
        const categoryId = requireString(args, "categoryId");
        const role = requireString(args, "role");
        const assignedAgent = optionalString(args, "assignedAgent");
        const plan = new PlanDocument(requestedPlanPath);
        const planPath = plan.planPath;
        const planId = planIdFromPath(planPath);
        syncStoredPlanPath(runtimeStore, planId, planPath);
        const category = categories.get(categoryId);
        if (!category) throw new Error(`Unknown category: ${categoryId}`);
        const nextStatus = (optionalString(args, "taskStatus") ?? (categoryId === "review" ? "running_quality_review" : "running_impl")) as TaskStatus;
        if (category.writePolicy === "lease-required" && nextStatus === "running_impl") {
          const activeLease = runtimeStore.getActiveWriteLease(planId, taskId);
          if (!activeLease) {
            throw new Error(`Cannot start ${taskId}: write lease required for category ${categoryId}`);
          }
        }
        runtimeStore.upsertPlanState({
          planId,
          planPath,
          currentWave: optionalString(args, "currentWave") ?? null,
          activeTaskId: taskId,
        });
        runtimeStore.upsertTaskState({
          planId,
          taskId,
          categoryId,
          status: nextStatus,
          assignedRole: role,
          agentId: assignedAgent ?? null,
        });
        const current = plan.readPlanState().executionStatus;
        plan.updateExecutionStatus({
          currentWave: optionalString(args, "currentWave") ?? current.currentWave,
          activeTask: taskId,
          blockers: "None",
        });
        plan.updateTaskMetadata(taskId, {
          taskStatus: nextStatus,
          assignedAgent: assignedAgent ?? "local-parent",
        });
        let task = plan.readPlanState().tasks.find((entry) => entry.id === taskId);
        if (!task) throw new Error(`Task not found in plan: ${taskId}`);
        let stepProgress = buildStepProgress(task, runtimeStore.getTaskState(planId, taskId));
        if (statusUsesStepPointer(nextStatus) && stepProgress.stepSyncStatus !== "all_steps_complete" && stepProgress.stepSyncStatus !== "step_in_progress" && stepProgress.nextStepLabel) {
          syncTaskCurrentStep(plan, runtimeStore, planId, taskId, stepProgress.nextStepLabel);
          task = plan.readPlanState().tasks.find((entry) => entry.id === taskId);
          if (!task) throw new Error(`Task not found in plan after step sync: ${taskId}`);
          stepProgress = buildStepProgress(task, runtimeStore.getTaskState(planId, taskId));
        }
        return result({
          plan_id: planId,
          task_id: taskId,
          task_status: nextStatus,
          ...stepProgressPayload(stepProgress),
        });
      },
    },
    {
      name: "orchestrator_acquire_write_lease",
      description: "Acquire an active write lease for a lease-required task.",
      inputSchema: {
        type: "object",
        properties: {
          planPath: { type: "string" },
          taskId: { type: "string" },
          holderAgentId: { type: "string" },
          scope: {
            type: "array",
            items: { type: "string" },
          },
        },
        required: ["planPath", "taskId", "holderAgentId", "scope"],
      },
      handler: async (args) => {
        const planPath = requireString(args, "planPath");
        const taskId = requireString(args, "taskId");
        const holderAgentId = requireString(args, "holderAgentId");
        const scope = requireStringArray(args, "scope");
        const planId = planIdFromPath(planPath);
        const lease = runtimeStore.acquireWriteLease({
          planId,
          taskId,
          holderAgentId,
          scope,
        });
        const current = runtimeStore.getTaskState(planId, taskId);
        if (current) {
          runtimeStore.upsertTaskState({
            ...current,
            planId,
            taskId,
            categoryId: current.categoryId,
            status: current.status,
            activeStepLabel: current.activeStepLabel,
            assignedRole: current.assignedRole,
            agentId: current.agentId,
            writeLeaseId: lease.leaseId,
            specReviewStatus: current.specReviewStatus,
            qualityReviewStatus: current.qualityReviewStatus,
            retryCount: current.retryCount,
            blockerType: null,
            blockerMessage: null,
          });
        }
        return result({
          plan_id: planId,
          task_id: taskId,
          lease_id: lease.leaseId,
          holder_agent_id: lease.holderAgentId,
          scope,
          status: lease.status,
        });
      },
    },
    {
      name: "orchestrator_release_write_lease",
      description: "Release an active write lease for a task.",
      inputSchema: {
        type: "object",
        properties: {
          planPath: { type: "string" },
          taskId: { type: "string" },
          leaseId: { type: "string" },
        },
        required: ["planPath", "taskId", "leaseId"],
      },
      handler: async (args) => {
        const planPath = requireString(args, "planPath");
        const taskId = requireString(args, "taskId");
        const leaseId = requireString(args, "leaseId");
        const planId = planIdFromPath(planPath);
        const lease = runtimeStore.releaseWriteLease(leaseId);
        const current = runtimeStore.getTaskState(planId, taskId);
        if (current) {
          runtimeStore.upsertTaskState({
            ...current,
            planId,
            taskId,
            categoryId: current.categoryId,
            status: current.status,
            activeStepLabel: current.activeStepLabel,
            assignedRole: current.assignedRole,
            agentId: current.agentId,
            writeLeaseId: null,
            specReviewStatus: current.specReviewStatus,
            qualityReviewStatus: current.qualityReviewStatus,
            retryCount: current.retryCount,
            blockerType: current.blockerType,
            blockerMessage: current.blockerMessage,
          });
        }
        return result({
          plan_id: planId,
          task_id: taskId,
          lease_id: lease.leaseId,
          status: lease.status,
          released_at: lease.releasedAt,
        });
      },
    },
    {
      name: "orchestrator_begin_step",
      description: "Record the current active step for a task.",
      inputSchema: {
        type: "object",
        properties: {
          planPath: { type: "string" },
          taskId: { type: "string" },
          stepLabel: { type: "string" },
        },
        required: ["planPath", "taskId", "stepLabel"],
      },
      handler: async (args) => {
        const requestedPlanPath = requireString(args, "planPath");
        const taskId = requireString(args, "taskId");
        const stepLabel = requireString(args, "stepLabel");
        const plan = new PlanDocument(requestedPlanPath);
        const planPath = plan.planPath;
        const planId = planIdFromPath(planPath);
        syncStoredPlanPath(runtimeStore, planId, planPath);
        const existing = runtimeStore.getTaskState(planId, taskId);
        if (!existing) throw new Error(`Task state missing: ${taskId}`);
        runtimeStore.upsertTaskState({
          ...existing,
          planId,
          taskId,
          categoryId: existing.categoryId,
          status: existing.status,
          activeStepLabel: stepLabel,
          assignedRole: existing.assignedRole,
          agentId: existing.agentId,
          writeLeaseId: existing.writeLeaseId,
          specReviewStatus: existing.specReviewStatus,
          qualityReviewStatus: existing.qualityReviewStatus,
          retryCount: existing.retryCount,
          blockerType: existing.blockerType,
          blockerMessage: existing.blockerMessage,
        });
        plan.updateTaskMetadata(taskId, { currentStep: stepLabel });
        const task = plan.readPlanState().tasks.find((entry) => entry.id === taskId);
        if (!task) throw new Error(`Task not found in plan: ${taskId}`);
        return result({
          plan_id: planId,
          task_id: taskId,
          current_step: stepLabel,
          ...stepProgressPayload(buildStepProgress(task, runtimeStore.getTaskState(planId, taskId))),
        });
      },
    },
    {
      name: "orchestrator_complete_step",
      description: "Check a task step checkbox and optionally record verification evidence.",
      inputSchema: {
        type: "object",
        properties: {
          planPath: { type: "string" },
          taskId: { type: "string" },
          stepLabel: { type: "string" },
          evidenceSummary: { type: "string" },
        },
        required: ["planPath", "taskId", "stepLabel"],
      },
      handler: async (args) => {
        const requestedPlanPath = requireString(args, "planPath");
        const taskId = requireString(args, "taskId");
        const stepLabel = requireString(args, "stepLabel");
        const evidenceSummary = optionalString(args, "evidenceSummary");
        const plan = new PlanDocument(requestedPlanPath);
        const planPath = plan.planPath;
        const planId = planIdFromPath(planPath);
        syncStoredPlanPath(runtimeStore, planId, planPath);
        plan.markStep(taskId, stepLabel, true);
        const taskAfterCheck = plan.readPlanState().tasks.find((entry) => entry.id === taskId);
        if (!taskAfterCheck) throw new Error(`Task not found in plan: ${taskId}`);
        const nextUncheckedStep = firstUncheckedStep(taskAfterCheck);
        syncTaskCurrentStep(plan, runtimeStore, planId, taskId, nextUncheckedStep?.label ?? null);
        if (evidenceSummary) {
          runtimeStore.recordEvidence({
            planId,
            taskId,
            kind: "step-completion",
            summary: evidenceSummary,
          });
        }
        const syncedTask = plan.readPlanState().tasks.find((entry) => entry.id === taskId);
        if (!syncedTask) throw new Error(`Task not found in plan after step completion: ${taskId}`);
        return result({
          plan_id: planId,
          task_id: taskId,
          step_label: stepLabel,
          checked: true,
          auto_advanced: nextUncheckedStep !== undefined,
          ...stepProgressPayload(buildStepProgress(syncedTask, runtimeStore.getTaskState(planId, taskId))),
        });
      },
    },
    {
      name: "orchestrator_record_subagent_run",
      description: "Record a subagent run against a task and bind the current agent id.",
      inputSchema: {
        type: "object",
        properties: {
          planPath: { type: "string" },
          taskId: { type: "string" },
          categoryId: { type: "string" },
          role: { type: "string" },
          agentId: { type: "string" },
          status: { type: "string" },
          summary: { type: "string" },
        },
        required: ["planPath", "taskId", "categoryId", "role", "agentId", "status"],
      },
      handler: async (args) => {
        const requestedPlanPath = requireString(args, "planPath");
        const taskId = requireString(args, "taskId");
        const categoryId = requireString(args, "categoryId");
        const role = requireString(args, "role");
        const agentId = requireString(args, "agentId");
        const status = requireString(args, "status");
        const summary = optionalString(args, "summary");
        const plan = new PlanDocument(requestedPlanPath);
        const planPath = plan.planPath;
        const planId = planIdFromPath(planPath);
        syncStoredPlanPath(runtimeStore, planId, planPath);
        const current = runtimeStore.getTaskState(planId, taskId);
        runtimeStore.recordTaskRun({ planId, taskId, role, agentId, status, summary });
        runtimeStore.upsertTaskState({
          planId,
          taskId,
          categoryId,
          status: current?.status ?? "running_impl",
          activeStepLabel: current?.activeStepLabel ?? null,
          assignedRole: role,
          agentId,
          writeLeaseId: current?.writeLeaseId ?? null,
          specReviewStatus: current?.specReviewStatus ?? "pending",
          qualityReviewStatus: current?.qualityReviewStatus ?? "pending",
          retryCount: current?.retryCount ?? 0,
          blockerType: current?.blockerType ?? null,
          blockerMessage: current?.blockerMessage ?? null,
        });
        plan.updateTaskMetadata(taskId, { assignedAgent: agentId });
        return result({ plan_id: planId, task_id: taskId, agent_id: agentId, recorded: true });
      },
    },
    {
      name: "orchestrator_record_review",
      description: "Record a spec or quality review result and update task metadata.",
      inputSchema: {
        type: "object",
        properties: {
          planPath: { type: "string" },
          taskId: { type: "string" },
          reviewType: { type: "string", enum: ["spec", "quality"] },
          result: { type: "string", enum: ["pass", "fail"] },
          notes: { type: "string" },
          reviewerAgentId: { type: "string" }
        },
        required: ["planPath", "taskId", "reviewType", "result"]
      },
      handler: async (args) => {
        const requestedPlanPath = requireString(args, "planPath");
        const taskId = requireString(args, "taskId");
        const reviewType = requireString(args, "reviewType");
        const reviewResult = requireString(args, "result") as ReviewStatus;
        const notes = optionalString(args, "notes");
        const reviewerAgentId = optionalString(args, "reviewerAgentId");
        if (reviewType !== "spec" && reviewType !== "quality") throw new Error("reviewType must be 'spec' or 'quality'");
        if (reviewResult !== "pass" && reviewResult !== "fail") throw new Error("result must be 'pass' or 'fail'");
        const plan = new PlanDocument(requestedPlanPath);
        const planPath = plan.planPath;
        const planId = planIdFromPath(planPath);
        syncStoredPlanPath(runtimeStore, planId, planPath);
        const current = runtimeStore.getTaskState(planId, taskId);
        if (!current) throw new Error(`Task state missing: ${taskId}`);
        if (reviewerAgentId && current.agentId && reviewerAgentId === current.agentId) {
          throw new Error("Reviewer must not reuse the implementer agent_id");
        }
        const specReviewStatus = reviewType === "spec" ? reviewResult : current.specReviewStatus;
        const qualityReviewStatus = reviewType === "quality" ? reviewResult : current.qualityReviewStatus;
        let nextTaskStatus: TaskStatus = current.status;
        if (reviewType === "spec") {
          nextTaskStatus = reviewResult === "pass" ? "running_quality_review" : "spec_failed";
        } else {
          nextTaskStatus = reviewResult === "pass" ? "impl_done" : "quality_failed";
        }
        runtimeStore.upsertTaskState({
          ...current,
          planId,
          taskId,
          categoryId: current.categoryId,
          status: nextTaskStatus,
          activeStepLabel: current.activeStepLabel,
          assignedRole: current.assignedRole,
          agentId: current.agentId,
          writeLeaseId: current.writeLeaseId,
          specReviewStatus,
          qualityReviewStatus,
          retryCount: current.retryCount,
          blockerType: current.blockerType,
          blockerMessage: current.blockerMessage,
        });
        if (notes) {
          runtimeStore.recordEvidence({
            planId,
            taskId,
            kind: `${reviewType}-review`,
            summary: notes,
          });
        }
        plan.updateTaskMetadata(taskId, {
          taskStatus: nextTaskStatus,
          specReviewStatus,
          qualityReviewStatus,
        });
        plan.updateExecutionStatus({
          lastReviewResult: `${taskId} ${reviewType} ${reviewResult}`,
          blockers: reviewResult === "fail" ? `${taskId} ${reviewType} review failed` : "None",
        });
        return result({ plan_id: planId, task_id: taskId, review_type: reviewType, result: reviewResult, task_status: nextTaskStatus });
      },
    },
    {
      name: "orchestrator_accept_task",
      description: "Accept a task after all steps and review gates pass, and check the top-level todo item.",
      inputSchema: {
        type: "object",
        properties: {
          planPath: { type: "string" },
          taskId: { type: "string" },
        },
        required: ["planPath", "taskId"],
      },
      handler: async (args) => {
        const requestedPlanPath = requireString(args, "planPath");
        const taskId = requireString(args, "taskId");
        const plan = new PlanDocument(requestedPlanPath);
        const planPath = plan.planPath;
        const planId = planIdFromPath(planPath);
        syncStoredPlanPath(runtimeStore, planId, planPath);
        const current = runtimeStore.getTaskState(planId, taskId);
        if (!current) throw new Error(`Task state missing: ${taskId}`);
        if (!plan.allStepsCompleted(taskId)) {
          throw new Error(`Cannot accept task ${taskId}: plan steps are not all checked`);
        }
        if (current.specReviewStatus !== "pass" || current.qualityReviewStatus !== "pass") {
          throw new Error(`Cannot accept task ${taskId}: both review gates must pass first`);
        }
        runtimeStore.upsertTaskState({
          ...current,
          planId,
          taskId,
          categoryId: current.categoryId,
          status: "accepted",
          activeStepLabel: null,
          assignedRole: current.assignedRole,
          agentId: current.agentId,
          writeLeaseId: current.writeLeaseId,
          specReviewStatus: current.specReviewStatus,
          qualityReviewStatus: current.qualityReviewStatus,
          retryCount: current.retryCount,
          blockerType: null,
          blockerMessage: null,
        });
        plan.updateTaskMetadata(taskId, { taskStatus: "accepted", currentStep: "none" });
        plan.markTopLevelTodo(taskId, true);
        return result({ plan_id: planId, task_id: taskId, accepted: true });
      },
    },
    {
      name: "orchestrator_check_doc_drift",
      description: "Check whether routing and architecture documents need synchronization based on changed paths.",
      inputSchema: {
        type: "object",
        properties: {
          changedPaths: {
            type: "array",
            items: { type: "string" },
          },
        },
        required: ["changedPaths"],
      },
      handler: async (args) => result(checkDocDrift(requireStringArray(args, "changedPaths"))),
    },
    {
      name: "orchestrator_watchdog_tick",
      description: "List stalled tasks that may require continuation or human attention.",
      inputSchema: {
        type: "object",
        properties: {
          planId: { type: "string" },
          olderThanMs: { type: "number" },
        },
        required: ["planId"],
      },
      handler: async (args) => {
        const planId = requireString(args, "planId");
        const stalled = runtimeStore.listStalledTasks(optionalNumber(args, "olderThanMs") ?? 15 * 60 * 1000)
          .filter((task) => task.planId === planId);
        const planStateRecord = runtimeStore.getPlanState(planId);
        const plan = planStateRecord ? new PlanDocument(planStateRecord.planPath) : undefined;
        const planTasks = plan ? plan.readPlanState().tasks : [];
        return result({
          plan_id: planId,
          stalled_tasks: stalled.map((task) => ({
            task_id: task.taskId,
            status: task.status,
            active_step: task.activeStepLabel,
            agent_id: task.agentId,
            suggested_action: deriveSuggestedAction({
              taskState: task,
              planTask: planTasks.find((planTask) => planTask.id === task.taskId),
              category: categories.get(task.categoryId),
              activeLeasePresent: task.writeLeaseId !== null || runtimeStore.getActiveWriteLease(task.planId, task.taskId) !== undefined,
            }),
          })),
        });
      },
    },
    {
      name: "orchestrator_next_action",
      description: "Derive the next deterministic parent-agent action from the active plan and runtime state.",
      inputSchema: {
        type: "object",
        properties: {
          planPath: { type: "string" },
        },
        required: ["planPath"],
      },
      handler: async (args) => {
        const requestedPlanPath = requireString(args, "planPath");
        const plan = new PlanDocument(requestedPlanPath);
        const planPath = plan.planPath;
        const planId = planIdFromPath(planPath);
        syncStoredPlanPath(runtimeStore, planId, planPath);
        const planState = plan.readPlanState();
        const nextTask = planState.tasks.find((task) => !task.todoChecked);
        if (!nextTask) {
          const openAcceptance = plan.uncheckedFinalAcceptanceItems();
          if (openAcceptance.length > 0) {
            return result(shapeNextActionPayload(planId, withDelegationMetadata(undefined, {
              action: "complete_final_acceptance",
              requires_write_lease: false,
              reason: `All top-level tasks are accepted, but final acceptance still has open items: ${openAcceptance.join(", ")}.`,
              open_acceptance_items: openAcceptance,
              ...emptyStepProgress(),
            })));
          }
          return result(shapeNextActionPayload(planId, withDelegationMetadata(undefined, {
            action: "complete_plan",
            reason: "All top-level tasks are already accepted.",
            requires_write_lease: false,
            ...emptyStepProgress(),
          })));
        }
        const taskState = runtimeStore.getTaskState(planId, nextTask.id);
        const category = categories.get(nextTask.category);
        const activeLease = runtimeStore.getActiveWriteLease(planId, nextTask.id);
        const action = deriveNextAction(
          nextTask,
          taskState,
          category,
          activeLease !== undefined,
          plan.uncheckedFinalAcceptanceItems(),
        );
        return result(shapeNextActionPayload(planId, action, nextTask.id));
      },
    },
    {
      name: "orchestrator_question_gate",
      description: "Decide whether a user-facing question is allowed or whether the parent should continue without asking.",
      inputSchema: {
        type: "object",
        properties: {
          questionCategory: { type: "string" },
          userExplicitlyRequested: { type: "boolean" },
          reason: { type: "string" },
        },
        required: ["questionCategory"],
      },
      handler: async (args) => {
        const questionCategory = requireString(args, "questionCategory");
        const userExplicitlyRequested = args.userExplicitlyRequested === true;
        const reason = optionalString(args, "reason") ?? "";
        const hardBlockers = new Set(["identity", "credential", "destructive", "conflict"]);

        if (questionCategory === "optional_expansion" && !userExplicitlyRequested) {
          return result({
            ask_user: false,
            blocker_type: "none",
            allowed_to_expand: false,
            recommended_action: "skip_optional_expansion",
            reason: reason || "Optional expansion was not explicitly requested by the user.",
          });
        }

        if (hardBlockers.has(questionCategory)) {
          return result({
            ask_user: true,
            blocker_type: questionCategory,
            allowed_to_expand: false,
            recommended_action: "ask_user",
            reason: reason || `A hard blocker of type ${questionCategory} requires user resolution.`,
          });
        }

        if (questionCategory === "system") {
          return result({
            ask_user: false,
            blocker_type: "system",
            allowed_to_expand: false,
            recommended_action: "retry_or_report",
            reason: reason || "System-level issues should be retried or reported instead of turned into user questions.",
          });
        }

        return result({
          ask_user: false,
          blocker_type: "none",
          allowed_to_expand: userExplicitlyRequested,
          recommended_action: userExplicitlyRequested ? "execute_requested_scope" : "record_assumption_and_continue",
          reason: reason || "No hard blocker requires a user-facing question.",
        });
      },
    },
    {
      name: "orchestrator_assess_subagent_completion",
      description: "Assess whether child output is sufficient for review, repair, acceptance, or more implementation.",
      inputSchema: {
        type: "object",
        properties: {
          planPath: { type: "string" },
          taskId: { type: "string" },
        },
        required: ["planPath", "taskId"],
      },
      handler: async (args) => {
        const requestedPlanPath = requireString(args, "planPath");
        const taskId = requireString(args, "taskId");
        const plan = new PlanDocument(requestedPlanPath);
        const planPath = plan.planPath;
        const planId = planIdFromPath(planPath);
        syncStoredPlanPath(runtimeStore, planId, planPath);
        const assessment = assessSubagentCompletion({
          plan,
          planId,
          taskId,
          runtimeStore,
        });
        return result({
          task_id: taskId,
          implementation_complete: assessment.implementationComplete,
          missing_steps: assessment.missingSteps,
          missing_evidence: assessment.missingEvidence,
          next_required_stage: assessment.nextRequiredStage,
          repair_role: assessment.repairRole,
          can_accept: assessment.canAccept,
        });
      },
    },
    {
      name: "orchestrator_completion_guard",
      description: "Fail closed when the parent tries to end work before plan completion reaches 100 percent.",
      inputSchema: {
        type: "object",
        properties: {
          planPath: { type: "string" },
        },
        required: ["planPath"],
      },
      handler: async (args) => {
        const requestedPlanPath = requireString(args, "planPath");
        const plan = new PlanDocument(requestedPlanPath);
        syncStoredPlanPath(runtimeStore, planIdFromPath(plan.planPath), plan.planPath);
        const planState = plan.readPlanState();
        const openTasks = planState.tasks.filter((task) => !task.todoChecked).map((task) => task.id);
        const openAcceptanceItems = plan.uncheckedFinalAcceptanceItems();
        const canFinish = openTasks.length === 0 && openAcceptanceItems.length === 0;
        return result({
          can_finish: canFinish,
          open_tasks: openTasks,
          open_acceptance_items: openAcceptanceItems,
          blocking_reason: canFinish
            ? "Plan completion is at 100 percent."
            : `Open tasks or final acceptance items remain: ${[...openTasks, ...openAcceptanceItems].join(", ")}`,
        });
      },
    },
  ];
}

function deriveSuggestedAction(input: {
  taskState: TaskStateRecord;
  planTask: PlanTask | undefined;
  category: CategoryDefinition | undefined;
  activeLeasePresent: boolean;
}): string {
  const { taskState, planTask, category, activeLeasePresent } = input;
  if (category?.writePolicy === "lease-required" && !activeLeasePresent) {
    return "acquire_write_lease";
  }
  if (planTask && (taskState.status === "running_impl" || taskState.status === "running_spec_review" || taskState.status === "running_quality_review")) {
    const stepProgress = buildStepProgress(planTask, taskState);
    if (stepProgress.stepSyncStatus === "needs_begin_step" || stepProgress.stepSyncStatus === "stale_current_step") {
      return "repair_step_sync";
    }
  }
  if (taskState.status === "spec_failed" || taskState.status === "quality_failed") {
    return "return_to_implementer";
  }
  if (taskState.status === "running_quality_review" || taskState.status === "running_spec_review") {
    return "re-run_review";
  }
  if (taskState.status === "blocked") {
    return "mark_blocked";
  }
  return "continue_same_agent";
}

function assessSubagentCompletion(input: {
  plan: PlanDocument;
  planId: string;
  taskId: string;
  runtimeStore: RuntimeStore;
}): {
  implementationComplete: boolean;
  missingSteps: string[];
  missingEvidence: boolean;
  nextRequiredStage: string;
  repairRole?: string;
  canAccept: boolean;
} {
  const { plan, planId, taskId, runtimeStore } = input;
  const planState = plan.readPlanState();
  const planTask = planState.tasks.find((task) => task.id === taskId);
  if (!planTask) throw new Error(`Task not found in plan: ${taskId}`);
  const taskState = runtimeStore.getTaskState(planId, taskId);
  const evidence = runtimeStore.listEvidenceForTask(planId, taskId);
  const missingSteps = planTask.steps.filter((step) => !step.checked).map((step) => step.label);
  const missingEvidence = evidence.length === 0;
  const implementationComplete = missingSteps.length === 0
    && taskState?.status !== "blocked"
    && taskState?.status !== "cancelled";

  let nextRequiredStage = "implementation";
  let repairRole: string | undefined;

  if (!implementationComplete) {
    nextRequiredStage = "implementation";
    repairRole = planTask.ownerRole;
  } else if (missingEvidence) {
    nextRequiredStage = "implementation_evidence";
    repairRole = planTask.ownerRole;
  } else if (planTask.specReviewStatus !== "pass") {
    nextRequiredStage = planTask.specReviewStatus === "fail" ? "repair" : "spec_review";
    repairRole = planTask.specReviewStatus === "fail" ? planTask.ownerRole : "harness-evaluator";
  } else if (planTask.qualityReviewStatus !== "pass") {
    nextRequiredStage = planTask.qualityReviewStatus === "fail" ? "repair" : "quality_review";
    repairRole = planTask.qualityReviewStatus === "fail" ? planTask.ownerRole : "harness-evaluator";
  } else if (planTask.todoChecked) {
    nextRequiredStage = "done";
  } else {
    nextRequiredStage = "accept";
  }

  const canAccept = implementationComplete
    && !missingEvidence
    && planTask.specReviewStatus === "pass"
    && planTask.qualityReviewStatus === "pass";

  return {
    implementationComplete,
    missingSteps,
    missingEvidence,
    nextRequiredStage,
    repairRole,
    canAccept,
  };
}

function deriveNextAction(
  planTask: PlanTask,
  taskState: TaskStateRecord | undefined,
  category: CategoryDefinition | undefined,
  activeLeasePresent: boolean,
  finalAcceptanceOpen: string[],
): {
  action: string;
  requiredRole?: string;
  requiresWriteLease: boolean;
  reason: string;
  requiresSubagent: boolean;
  dispatchRole?: string;
  interventionReason: string;
  dispatchMode: string;
  currentStepLabel?: string;
  currentStepText?: string;
  nextStepLabel?: string;
  nextStepText?: string;
  remainingStepCount: number;
  stepSyncStatus: StepSyncStatus;
  stepSyncAction: StepSyncAction;
} {
  const requiresWriteLease = category?.writePolicy === "lease-required";
  const stepProgress = buildStepProgress(planTask, taskState);
  if (requiresWriteLease && !activeLeasePresent) {
    return withDelegationMetadata(category, {
      action: "acquire_write_lease",
      requiredRole: planTask.ownerRole,
      requiresWriteLease: true,
      reason: `Task ${planTask.id} belongs to a lease-required category and has no active lease.`,
      ...stepProgress,
    });
  }

  if (taskState?.status === "impl_done" && (planTask.specReviewStatus === "pending" || planTask.specReviewStatus === "fail")) {
    return withDelegationMetadata(category, {
      action: planTask.specReviewStatus === "fail" ? "repair_and_re_review" : "run_spec_review",
      requiredRole: planTask.specReviewStatus === "fail" ? planTask.ownerRole : "harness-evaluator",
      requiresWriteLease,
      reason: planTask.specReviewStatus === "fail"
        ? `Task ${planTask.id} failed spec review and must return to implementation before review repeats.`
        : `Task ${planTask.id} needs spec review before it can continue.`,
      ...stepProgress,
    });
  }

  if (taskState?.status === "impl_done" && planTask.specReviewStatus === "pass" && (planTask.qualityReviewStatus === "pending" || planTask.qualityReviewStatus === "fail")) {
    return withDelegationMetadata(category, {
      action: planTask.qualityReviewStatus === "fail" ? "repair_and_re_review" : "run_quality_review",
      requiredRole: planTask.qualityReviewStatus === "fail" ? planTask.ownerRole : "harness-evaluator",
      requiresWriteLease,
      reason: planTask.qualityReviewStatus === "fail"
        ? `Task ${planTask.id} failed quality review and must return to implementation before review repeats.`
        : `Task ${planTask.id} needs quality review before it can be accepted.`,
      ...stepProgress,
    });
  }

  if (taskState?.status === "spec_failed" || taskState?.status === "quality_failed") {
    return withDelegationMetadata(category, {
      action: "return_to_implementer",
      requiredRole: taskState.assignedRole ?? planTask.ownerRole,
      requiresWriteLease,
      reason: `Task ${planTask.id} failed a review gate and must return to implementation.`,
      ...stepProgress,
    });
  }

  if (planTask.specReviewStatus === "pass" && planTask.qualityReviewStatus === "pass" && planTask.steps.every((step) => step.checked)) {
    return withDelegationMetadata(category, {
      action: "accept_task",
      requiredRole: undefined,
      requiresWriteLease,
      reason: `Task ${planTask.id} has all steps checked and both review gates passed.`,
      ...stepProgress,
    });
  }

  if (planTask.steps.every((step) => step.checked) && planTask.specReviewStatus === "pending") {
    return withDelegationMetadata(category, {
      action: "run_spec_review",
      requiredRole: "harness-evaluator",
      requiresWriteLease,
      reason: `Task ${planTask.id} has completed implementation steps and now needs spec review.`,
      ...stepProgress,
    });
  }

  if (planTask.steps.every((step) => step.checked) && planTask.specReviewStatus === "pass" && planTask.qualityReviewStatus === "pending") {
    return withDelegationMetadata(category, {
      action: "run_quality_review",
      requiredRole: "harness-evaluator",
      requiresWriteLease,
      reason: `Task ${planTask.id} passed spec review and now needs quality review.`,
      ...stepProgress,
    });
  }

  if (planTask.todoChecked && finalAcceptanceOpen.length > 0) {
    return withDelegationMetadata(category, {
      action: "complete_final_acceptance",
      requiredRole: undefined,
      requiresWriteLease: false,
      reason: `All top-level tasks are accepted, but final acceptance still has open items: ${finalAcceptanceOpen.join(", ")}.`,
      ...stepProgress,
    });
  }

  if (taskState?.status === "running_impl") {
    return withDelegationMetadata(category, {
      action: "continue_same_agent",
      requiredRole: taskState.assignedRole ?? planTask.ownerRole,
      requiresWriteLease,
      reason: `Task ${planTask.id} is already in implementation progress.`,
      ...stepProgress,
    });
  }

  if (taskState?.status === "running_spec_review" || taskState?.status === "running_quality_review") {
    return withDelegationMetadata(category, {
      action: "continue_same_agent",
      requiredRole: taskState.assignedRole ?? "harness-evaluator",
      requiresWriteLease,
      reason: `Task ${planTask.id} already has an in-progress review assignment.`,
      ...stepProgress,
    });
  }

  return withDelegationMetadata(category, {
    action: "dispatch_task",
    requiredRole: planTask.ownerRole,
    requiresWriteLease,
    reason: `Task ${planTask.id} is the next incomplete top-level task in the plan.`,
    ...stepProgress,
  });
}

function withDelegationMetadata(
  category: CategoryDefinition | undefined,
  action: {
    task_id?: string;
    open_acceptance_items?: string[];
    action: string;
    requiredRole?: string;
    requiresWriteLease?: boolean;
    requires_write_lease?: boolean;
    reason: string;
    currentStepLabel?: string;
    currentStepText?: string;
    nextStepLabel?: string;
    nextStepText?: string;
    remainingStepCount: number;
    stepSyncStatus: StepSyncStatus;
    stepSyncAction: StepSyncAction;
  },
): {
  task_id?: string;
  open_acceptance_items?: string[];
  action: string;
  requiredRole?: string;
  requiresWriteLease: boolean;
  requires_write_lease: boolean;
  reason: string;
  requiresSubagent: boolean;
  dispatchRole?: string;
  interventionReason: string;
  dispatchMode: string;
  currentStepLabel?: string;
  currentStepText?: string;
  nextStepLabel?: string;
  nextStepText?: string;
  remainingStepCount: number;
  stepSyncStatus: StepSyncStatus;
  stepSyncAction: StepSyncAction;
} {
  const parentOnlyActions = new Set(["acquire_write_lease", "accept_task", "complete_final_acceptance", "complete_plan"]);
  const delegationPreference = category?.delegationPreference ?? "parent-only";
  const requiresWriteLease = action.requiresWriteLease ?? action.requires_write_lease ?? false;
  const dispatchMode = deriveDispatchMode(category, delegationPreference !== "parent-only" && !!action.requiredRole && !parentOnlyActions.has(action.action));

  if (parentOnlyActions.has(action.action) || !action.requiredRole || delegationPreference === "parent-only") {
    return {
      ...action,
      requiresWriteLease,
      requires_write_lease: requiresWriteLease,
      requiresSubagent: false,
      dispatchRole: undefined,
      interventionReason: parentOnlyActions.has(action.action)
        ? `Action ${action.action} is a parent-owned control-plane step.`
        : delegationPreference === "parent-only"
          ? `Category ${category?.id ?? "unknown"} is configured for parent-local execution.`
          : "No child role is required for this action.",
      dispatchMode: "parent-local",
    };
  }

  return {
    ...action,
    requiresWriteLease,
    requires_write_lease: requiresWriteLease,
    requiresSubagent: true,
    dispatchRole: action.requiredRole,
    interventionReason: delegationPreference === "subagent-required"
      ? `Category ${category?.id ?? "unknown"} requires subagent execution for normal task work.`
      : `Category ${category?.id ?? "unknown"} prefers subagent execution by default.`,
      dispatchMode,
    };
  }

function deriveDispatchMode(
  category: CategoryDefinition | undefined,
  requiresSubagent: boolean,
): string {
  if (!requiresSubagent) {
    return "parent-local";
  }
  switch (category?.parallelism) {
    case "parallel":
      return "parallel-subagents";
    case "write-scope":
      return "write-scope-subagent";
    default:
      return "single-subagent";
  }
}

function shapeNextActionPayload(
  planId: string,
  action: {
    task_id?: string;
    open_acceptance_items?: string[];
    action: string;
    requiredRole?: string;
    requiresWriteLease: boolean;
    reason: string;
    requiresSubagent: boolean;
    dispatchRole?: string;
    interventionReason: string;
    dispatchMode: string;
    currentStepLabel?: string;
    currentStepText?: string;
    nextStepLabel?: string;
    nextStepText?: string;
    remainingStepCount: number;
    stepSyncStatus: StepSyncStatus;
    stepSyncAction: StepSyncAction;
  },
  taskId?: string,
): Record<string, unknown> {
  return {
    plan_id: planId,
    task_id: taskId ?? action.task_id,
    action: action.action,
    required_role: action.requiredRole,
    requires_write_lease: action.requiresWriteLease,
    reason: action.reason,
    requires_subagent: action.requiresSubagent,
    dispatch_role: action.dispatchRole,
    intervention_reason: action.interventionReason,
    dispatch_mode: action.dispatchMode,
    open_acceptance_items: action.open_acceptance_items,
    current_step_label: action.currentStepLabel,
    current_step_text: action.currentStepText,
    next_step_label: action.nextStepLabel,
    next_step_text: action.nextStepText,
    remaining_step_count: action.remainingStepCount,
    step_sync_status: action.stepSyncStatus,
    step_sync_action: action.stepSyncAction,
  };
}
