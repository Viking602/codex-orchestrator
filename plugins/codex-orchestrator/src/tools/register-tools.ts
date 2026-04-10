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
        return result({ plan_id: planId, task_id: taskId, task_status: nextStatus });
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
            suggested_action: deriveSuggestedAction({
              taskState: task,
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
        const planPath = requireString(args, "planPath");
        const plan = new PlanDocument(planPath);
        const planId = planIdFromPath(planPath);
        const planState = plan.readPlanState();
        const nextTask = planState.tasks.find((task) => !task.todoChecked);
        if (!nextTask) {
          const openAcceptance = plan.uncheckedFinalAcceptanceItems();
          if (openAcceptance.length > 0) {
            return result({
              plan_id: planId,
              action: "complete_final_acceptance",
              requires_write_lease: false,
              reason: `All top-level tasks are accepted, but final acceptance still has open items: ${openAcceptance.join(", ")}.`,
              open_acceptance_items: openAcceptance,
            });
          }
          return result({
            plan_id: planId,
            action: "complete_plan",
            reason: "All top-level tasks are already accepted.",
          });
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
        return result({
          plan_id: planId,
          task_id: nextTask.id,
          action: action.action,
          required_role: action.requiredRole,
          requires_write_lease: action.requiresWriteLease,
          reason: action.reason,
        });
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
        const planPath = requireString(args, "planPath");
        const taskId = requireString(args, "taskId");
        const plan = new PlanDocument(planPath);
        const planId = planIdFromPath(planPath);
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
        const planPath = requireString(args, "planPath");
        const plan = new PlanDocument(planPath);
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
  category: CategoryDefinition | undefined;
  activeLeasePresent: boolean;
}): string {
  const { taskState, category, activeLeasePresent } = input;
  if (category?.writePolicy === "lease-required" && !activeLeasePresent) {
    return "acquire_write_lease";
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
} {
  const requiresWriteLease = category?.writePolicy === "lease-required";
  if (requiresWriteLease && !activeLeasePresent) {
    return {
      action: "acquire_write_lease",
      requiredRole: planTask.ownerRole,
      requiresWriteLease: true,
      reason: `Task ${planTask.id} belongs to a lease-required category and has no active lease.`,
    };
  }

  if (taskState?.status === "impl_done" && (planTask.specReviewStatus === "pending" || planTask.specReviewStatus === "fail")) {
    return {
      action: planTask.specReviewStatus === "fail" ? "repair_and_re_review" : "run_spec_review",
      requiredRole: planTask.specReviewStatus === "fail" ? planTask.ownerRole : "harness-evaluator",
      requiresWriteLease,
      reason: planTask.specReviewStatus === "fail"
        ? `Task ${planTask.id} failed spec review and must return to implementation before review repeats.`
        : `Task ${planTask.id} needs spec review before it can continue.`,
    };
  }

  if (taskState?.status === "impl_done" && planTask.specReviewStatus === "pass" && (planTask.qualityReviewStatus === "pending" || planTask.qualityReviewStatus === "fail")) {
    return {
      action: planTask.qualityReviewStatus === "fail" ? "repair_and_re_review" : "run_quality_review",
      requiredRole: planTask.qualityReviewStatus === "fail" ? planTask.ownerRole : "harness-evaluator",
      requiresWriteLease,
      reason: planTask.qualityReviewStatus === "fail"
        ? `Task ${planTask.id} failed quality review and must return to implementation before review repeats.`
        : `Task ${planTask.id} needs quality review before it can be accepted.`,
    };
  }

  if (taskState?.status === "spec_failed" || taskState?.status === "quality_failed") {
    return {
      action: "return_to_implementer",
      requiredRole: taskState.assignedRole ?? planTask.ownerRole,
      requiresWriteLease,
      reason: `Task ${planTask.id} failed a review gate and must return to implementation.`,
    };
  }

  if (planTask.specReviewStatus === "pass" && planTask.qualityReviewStatus === "pass" && planTask.steps.every((step) => step.checked)) {
    return {
      action: "accept_task",
      requiredRole: undefined,
      requiresWriteLease,
      reason: `Task ${planTask.id} has all steps checked and both review gates passed.`,
    };
  }

  if (planTask.steps.every((step) => step.checked) && planTask.specReviewStatus === "pending") {
    return {
      action: "run_spec_review",
      requiredRole: "harness-evaluator",
      requiresWriteLease,
      reason: `Task ${planTask.id} has completed implementation steps and now needs spec review.`,
    };
  }

  if (planTask.steps.every((step) => step.checked) && planTask.specReviewStatus === "pass" && planTask.qualityReviewStatus === "pending") {
    return {
      action: "run_quality_review",
      requiredRole: "harness-evaluator",
      requiresWriteLease,
      reason: `Task ${planTask.id} passed spec review and now needs quality review.`,
    };
  }

  if (planTask.todoChecked && finalAcceptanceOpen.length > 0) {
    return {
      action: "complete_final_acceptance",
      requiredRole: undefined,
      requiresWriteLease: false,
      reason: `All top-level tasks are accepted, but final acceptance still has open items: ${finalAcceptanceOpen.join(", ")}.`,
    };
  }

  if (taskState?.status === "running_impl") {
    return {
      action: "continue_same_agent",
      requiredRole: taskState.assignedRole ?? planTask.ownerRole,
      requiresWriteLease,
      reason: `Task ${planTask.id} is already in implementation progress.`,
    };
  }

  return {
    action: "dispatch_task",
    requiredRole: planTask.ownerRole,
    requiresWriteLease,
    reason: `Task ${planTask.id} is the next incomplete top-level task in the plan.`,
  };
}
