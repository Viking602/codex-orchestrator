export type ReviewStatus = "pending" | "pass" | "fail";
export type DelegationPreference = "parent-only" | "prefer-subagent" | "subagent-required";

export type TaskStatus =
  | "planned"
  | "ready"
  | "running_impl"
  | "impl_done"
  | "running_spec_review"
  | "spec_failed"
  | "running_quality_review"
  | "quality_failed"
  | "accepted"
  | "blocked"
  | "cancelled";

export interface CategoryDefinition {
  id: string;
  intent: string;
  preferredRole: string;
  allowedRoles: string[];
  writePolicy: string;
  requiresPlan: boolean;
  requiresSpecReview: boolean;
  requiresQualityReview: boolean;
  parallelism: string;
  delegationPreference: DelegationPreference;
  reusePolicy: string;
  completionContract: string[];
}

export interface CategoryResolution {
  categoryId: string;
  reason: string;
  category: CategoryDefinition;
}

export interface PlanExecutionStatus {
  currentWave: string;
  activeTask: string;
  blockers: string;
  lastReviewResult: string;
}

export interface PlanStep {
  label: string;
  text: string;
  checked: boolean;
}

export interface FinalAcceptanceItem {
  text: string;
  checked: boolean;
}

export interface PlanTask {
  id: string;
  title: string;
  category: string;
  ownerRole: string;
  taskStatus: string;
  currentStep: string;
  specReviewStatus: ReviewStatus | string;
  qualityReviewStatus: ReviewStatus | string;
  assignedAgent: string;
  todoChecked: boolean;
  steps: PlanStep[];
}

export interface WriteLeaseRecord {
  leaseId: string;
  planId: string;
  taskId: string;
  holderAgentId: string;
  scopeJson: string;
  status: "active" | "released";
  createdAt: string;
  releasedAt: string | null;
}

export interface TaskStateRecord {
  planId: string;
  taskId: string;
  categoryId: string;
  status: TaskStatus;
  activeStepLabel: string | null;
  assignedRole: string | null;
  agentId: string | null;
  writeLeaseId: string | null;
  specReviewStatus: ReviewStatus;
  qualityReviewStatus: ReviewStatus;
  retryCount: number;
  blockerType: string | null;
  blockerMessage: string | null;
  updatedAt: string;
}

export interface PlanStateRecord {
  planId: string;
  planPath: string;
  specPath: string | null;
  currentWave: string | null;
  activeTaskId: string | null;
  lastReviewResult: string | null;
  updatedAt: string;
}
