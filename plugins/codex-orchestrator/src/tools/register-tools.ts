import { basename } from "node:path";
import { CategoryRegistry } from "../services/category-registry.ts";
import { checkDocDrift } from "../services/doc-drift.ts";
import { PlanDocument } from "../services/plan-document.ts";
import { RuntimeStore } from "../services/runtime-store.ts";
import type { ReviewStatus, TaskStatus } from "../types.ts";

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
        return result(plan.readPlanState());
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
        const planPath = requireString(args, "planPath");
        const taskId = requireString(args, "taskId");
        const categoryId = requireString(args, "categoryId");
        const role = requireString(args, "role");
        const assignedAgent = optionalString(args, "assignedAgent");
        const plan = new PlanDocument(planPath);
        const planId = planIdFromPath(planPath);
        const category = categories.get(categoryId);
        if (!category) throw new Error(`Unknown category: ${categoryId}`);
        const nextStatus = (optionalString(args, "taskStatus") ?? (categoryId === "review" ? "running_quality_review" : "running_impl")) as TaskStatus;
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
        return result({ plan_id: planId, task_id: taskId, task_status: nextStatus });
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
        const planPath = requireString(args, "planPath");
        const taskId = requireString(args, "taskId");
        const stepLabel = requireString(args, "stepLabel");
        const plan = new PlanDocument(planPath);
        const planId = planIdFromPath(planPath);
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
        return result({ plan_id: planId, task_id: taskId, current_step: stepLabel });
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
        const planPath = requireString(args, "planPath");
        const taskId = requireString(args, "taskId");
        const stepLabel = requireString(args, "stepLabel");
        const evidenceSummary = optionalString(args, "evidenceSummary");
        const plan = new PlanDocument(planPath);
        const planId = planIdFromPath(planPath);
        plan.markStep(taskId, stepLabel, true);
        plan.updateTaskMetadata(taskId, { currentStep: "none" });
        const existing = runtimeStore.getTaskState(planId, taskId);
        if (existing) {
          runtimeStore.upsertTaskState({
            ...existing,
            planId,
            taskId,
            categoryId: existing.categoryId,
            status: existing.status,
            activeStepLabel: null,
            assignedRole: existing.assignedRole,
            agentId: existing.agentId,
            writeLeaseId: existing.writeLeaseId,
            specReviewStatus: existing.specReviewStatus,
            qualityReviewStatus: existing.qualityReviewStatus,
            retryCount: existing.retryCount,
            blockerType: existing.blockerType,
            blockerMessage: existing.blockerMessage,
          });
        }
        if (evidenceSummary) {
          runtimeStore.recordEvidence({
            planId,
            taskId,
            kind: "step-completion",
            summary: evidenceSummary,
          });
        }
        return result({ plan_id: planId, task_id: taskId, step_label: stepLabel, checked: true });
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
        const planPath = requireString(args, "planPath");
        const taskId = requireString(args, "taskId");
        const categoryId = requireString(args, "categoryId");
        const role = requireString(args, "role");
        const agentId = requireString(args, "agentId");
        const status = requireString(args, "status");
        const summary = optionalString(args, "summary");
        const plan = new PlanDocument(planPath);
        const planId = planIdFromPath(planPath);
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
        const planPath = requireString(args, "planPath");
        const taskId = requireString(args, "taskId");
        const reviewType = requireString(args, "reviewType");
        const reviewResult = requireString(args, "result") as ReviewStatus;
        const notes = optionalString(args, "notes");
        const reviewerAgentId = optionalString(args, "reviewerAgentId");
        if (reviewType !== "spec" && reviewType !== "quality") throw new Error("reviewType must be 'spec' or 'quality'");
        if (reviewResult !== "pass" && reviewResult !== "fail") throw new Error("result must be 'pass' or 'fail'");
        const plan = new PlanDocument(planPath);
        const planId = planIdFromPath(planPath);
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
        const planPath = requireString(args, "planPath");
        const taskId = requireString(args, "taskId");
        const plan = new PlanDocument(planPath);
        const planId = planIdFromPath(planPath);
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
        return result({
          plan_id: planId,
          stalled_tasks: stalled.map((task) => ({
            task_id: task.taskId,
            status: task.status,
            active_step: task.activeStepLabel,
            agent_id: task.agentId,
            suggested_action: task.status === "spec_failed" || task.status === "quality_failed"
              ? "return_to_implementer"
              : "continue_same_agent",
          })),
        });
      },
    },
  ];
}
