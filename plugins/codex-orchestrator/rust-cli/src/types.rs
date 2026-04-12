use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ReviewStatus {
    Pending,
    Pass,
    Fail,
}

impl ReviewStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Pass => "pass",
            Self::Fail => "fail",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DelegationPreference {
    ParentOnly,
    PreferSubagent,
    SubagentRequired,
}

impl DelegationPreference {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ParentOnly => "parent-only",
            Self::PreferSubagent => "prefer-subagent",
            Self::SubagentRequired => "subagent-required",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    Planned,
    Ready,
    RunningImpl,
    ImplDone,
    RunningSpecReview,
    SpecFailed,
    RunningQualityReview,
    QualityFailed,
    Accepted,
    Blocked,
    Cancelled,
}

impl TaskStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Planned => "planned",
            Self::Ready => "ready",
            Self::RunningImpl => "running_impl",
            Self::ImplDone => "impl_done",
            Self::RunningSpecReview => "running_spec_review",
            Self::SpecFailed => "spec_failed",
            Self::RunningQualityReview => "running_quality_review",
            Self::QualityFailed => "quality_failed",
            Self::Accepted => "accepted",
            Self::Blocked => "blocked",
            Self::Cancelled => "cancelled",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CategoryDefinition {
    pub id: String,
    pub intent: String,
    pub preferred_role: String,
    pub allowed_roles: Vec<String>,
    pub write_policy: String,
    pub requires_plan: bool,
    pub requires_spec_review: bool,
    pub requires_quality_review: bool,
    pub parallelism: String,
    pub delegation_preference: DelegationPreference,
    pub reuse_policy: String,
    pub completion_contract: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CategoryResolution {
    pub category_id: String,
    pub reason: String,
    pub category: CategoryDefinition,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanExecutionStatus {
    pub current_wave: String,
    pub active_task: String,
    pub blockers: String,
    pub last_review_result: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanStep {
    pub label: String,
    pub text: String,
    pub checked: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FinalAcceptanceItem {
    pub text: String,
    pub checked: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanTask {
    pub id: String,
    pub title: String,
    pub category: String,
    pub owner_role: String,
    pub task_status: String,
    pub current_step: String,
    pub spec_review_status: String,
    pub quality_review_status: String,
    pub assigned_agent: String,
    pub todo_checked: bool,
    pub steps: Vec<PlanStep>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WriteLeaseRecord {
    pub lease_id: String,
    pub plan_id: String,
    pub task_id: String,
    pub holder_agent_id: String,
    pub scope_json: String,
    pub status: String,
    pub created_at: String,
    pub released_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskStateRecord {
    pub plan_id: String,
    pub task_id: String,
    pub category_id: String,
    pub status: String,
    pub active_step_label: Option<String>,
    pub assigned_role: Option<String>,
    pub agent_id: Option<String>,
    pub write_lease_id: Option<String>,
    pub spec_review_status: String,
    pub quality_review_status: String,
    pub retry_count: i64,
    pub blocker_type: Option<String>,
    pub blocker_message: Option<String>,
    pub updated_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanStateRecord {
    pub plan_id: String,
    pub plan_path: String,
    pub spec_path: Option<String>,
    pub current_wave: Option<String>,
    pub active_task_id: Option<String>,
    pub last_review_result: Option<String>,
    pub updated_at: String,
}
