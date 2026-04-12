use std::{collections::HashSet, path::Path};

use anyhow::{anyhow, Result};
use serde::Serialize;
use serde_json::{json, Map, Value};

use crate::{
    category_registry::CategoryRegistry,
    doc_drift::check_doc_drift,
    plan_document::{PlanDocument, PlanState},
    runtime_store::{PlanStateUpsertInput, RuntimeStore, TaskStateUpsertInput},
    types::{CategoryDefinition, DelegationPreference, PlanTask, PlanStep, TaskStateRecord},
};

pub struct AppContext {
    pub categories: CategoryRegistry,
    pub runtime_store: RuntimeStore,
}

pub struct ToolSpec {
    pub name: &'static str,
    pub description: &'static str,
    pub input_schema: Value,
}

#[derive(Debug, Clone, Serialize)]
struct StepProgressState {
    current_step_label: Option<String>,
    current_step_text: Option<String>,
    next_step_label: Option<String>,
    next_step_text: Option<String>,
    remaining_step_count: usize,
    step_sync_status: String,
    step_sync_action: String,
}

#[derive(Debug, Clone, Serialize)]
struct CodexTodoItem {
    step: String,
    status: String,
}

#[derive(Debug, Clone, Serialize)]
struct BlockingControlPlaneAction {
    action: String,
    tool_name: String,
    reason: String,
    task_id: String,
    category_id: Option<String>,
    role: Option<String>,
    task_status: Option<String>,
    step_label: Option<String>,
}

#[derive(Debug, Clone)]
struct CodexTodoMirror {
    items: Vec<CodexTodoItem>,
    active_task_id: Option<String>,
    active_task_title: Option<String>,
    current_step_label: Option<String>,
    current_step_text: Option<String>,
    remaining_step_count: usize,
    step_sync_status: String,
    step_sync_action: String,
    open_acceptance_items: Vec<String>,
}

#[derive(Debug, Clone)]
struct ActionState {
    task_id: Option<String>,
    open_acceptance_items: Vec<String>,
    action: String,
    category_id: Option<String>,
    required_role: Option<String>,
    requires_write_lease: bool,
    reason: String,
    requires_subagent: bool,
    dispatch_role: Option<String>,
    intervention_reason: String,
    dispatch_mode: String,
    task_session_mode: String,
    task_session_key: Option<String>,
    continue_agent_id: Option<String>,
    subagent_tool_action: String,
    subagent_agent_type: Option<String>,
    subagent_dispatch_message: Option<String>,
    blocking_control_plane_actions: Vec<BlockingControlPlaneAction>,
    child_execution_mode: String,
    child_execution_label: Option<String>,
    child_execution_text: Option<String>,
    step_progress: StepProgressState,
}

#[derive(Debug, Clone, Serialize)]
struct ParallelDispatchEntry {
    task_id: String,
    title: String,
    action: String,
    category_id: String,
    dispatch_role: String,
    dispatch_mode: String,
    task_session_mode: String,
    task_session_key: String,
    continue_agent_id: Option<String>,
    subagent_tool_action: String,
    subagent_agent_type: String,
    subagent_dispatch_message: String,
    blocking_control_plane_actions: Vec<BlockingControlPlaneAction>,
    child_execution_mode: String,
    child_execution_label: Option<String>,
    child_execution_text: Option<String>,
    requires_write_lease: bool,
    dispatch_scope: Vec<String>,
    depends_on: Vec<String>,
    reason: String,
}

#[derive(Debug, Clone)]
struct ParallelBatchPlan {
    top_level_action: String,
    entries: Vec<ParallelDispatchEntry>,
}

#[derive(Debug, Clone)]
struct RawActionState {
    task_id: Option<String>,
    open_acceptance_items: Vec<String>,
    action: String,
    category_id: Option<String>,
    required_role: Option<String>,
    requires_write_lease: bool,
    reason: String,
    step_progress: StepProgressState,
}

#[derive(Debug)]
struct CompletionAssessment {
    implementation_complete: bool,
    missing_steps: Vec<String>,
    missing_evidence: bool,
    next_required_stage: String,
    repair_role: Option<String>,
    can_accept: bool,
}

#[derive(Debug, Clone)]
struct TaskAcceptanceOutcome {
    next_active_task_id: Option<String>,
}

pub fn tool_specs() -> Vec<ToolSpec> {
    vec![
        ToolSpec {
            name: "orchestrator_resolve_category",
            description: "Resolve a workflow category and allowed roles for a task.",
            input_schema: json!({
                "type": "object",
                "properties": {
                    "title": { "type": "string" },
                    "description": { "type": "string" },
                    "explicitCategory": { "type": "string" }
                },
                "required": ["title", "description"]
            }),
        },
        ToolSpec {
            name: "orchestrator_read_plan_state",
            description: "Read the current execution status and parsed tasks from an implementation plan.",
            input_schema: json!({
                "type": "object",
                "properties": { "planPath": { "type": "string" } },
                "required": ["planPath"]
            }),
        },
        ToolSpec {
            name: "orchestrator_export_codex_todo",
            description: "Export the active implementation plan as mirror-ready items for Codex native update_plan.",
            input_schema: json!({
                "type": "object",
                "properties": { "planPath": { "type": "string" } },
                "required": ["planPath"]
            }),
        },
        ToolSpec {
            name: "orchestrator_begin_task",
            description: "Mark a task as started in runtime state and in the active plan.",
            input_schema: json!({
                "type": "object",
                "properties": {
                    "planPath": { "type": "string" },
                    "taskId": { "type": "string" },
                    "categoryId": { "type": "string" },
                    "role": { "type": "string" },
                    "taskStatus": { "type": "string" },
                    "currentWave": { "type": "string" },
                    "assignedAgent": { "type": "string" }
                },
                "required": ["planPath", "taskId", "categoryId", "role"]
            }),
        },
        ToolSpec {
            name: "orchestrator_acquire_write_lease",
            description: "Acquire an active write lease for a lease-required task.",
            input_schema: json!({
                "type": "object",
                "properties": {
                    "planPath": { "type": "string" },
                    "taskId": { "type": "string" },
                    "holderAgentId": { "type": "string" },
                    "scope": { "type": "array", "items": { "type": "string" } }
                },
                "required": ["planPath", "taskId", "holderAgentId", "scope"]
            }),
        },
        ToolSpec {
            name: "orchestrator_release_write_lease",
            description: "Release an active write lease for a task.",
            input_schema: json!({
                "type": "object",
                "properties": {
                    "planPath": { "type": "string" },
                    "taskId": { "type": "string" },
                    "leaseId": { "type": "string" }
                },
                "required": ["planPath", "taskId", "leaseId"]
            }),
        },
        ToolSpec {
            name: "orchestrator_begin_step",
            description: "Record the current active step for a task.",
            input_schema: json!({
                "type": "object",
                "properties": {
                    "planPath": { "type": "string" },
                    "taskId": { "type": "string" },
                    "stepLabel": { "type": "string" }
                },
                "required": ["planPath", "taskId", "stepLabel"]
            }),
        },
        ToolSpec {
            name: "orchestrator_complete_step",
            description: "Check a task step checkbox and optionally record verification evidence.",
            input_schema: json!({
                "type": "object",
                "properties": {
                    "planPath": { "type": "string" },
                    "taskId": { "type": "string" },
                    "stepLabel": { "type": "string" },
                    "evidenceSummary": { "type": "string" }
                },
                "required": ["planPath", "taskId", "stepLabel"]
            }),
        },
        ToolSpec {
            name: "orchestrator_record_subagent_run",
            description: "Record a subagent run against a task and bind the current agent id.",
            input_schema: json!({
                "type": "object",
                "properties": {
                    "planPath": { "type": "string" },
                    "taskId": { "type": "string" },
                    "categoryId": { "type": "string" },
                    "role": { "type": "string" },
                    "agentId": { "type": "string" },
                    "status": { "type": "string" },
                    "summary": { "type": "string" }
                },
                "required": ["planPath", "taskId", "categoryId", "role", "agentId", "status"]
            }),
        },
        ToolSpec {
            name: "orchestrator_record_review",
            description: "Record a spec or quality review result, update task metadata, and immediately accept a terminal-ready task.",
            input_schema: json!({
                "type": "object",
                "properties": {
                    "planPath": { "type": "string" },
                    "taskId": { "type": "string" },
                    "reviewType": { "type": "string", "enum": ["spec", "quality"] },
                    "result": { "type": "string", "enum": ["pass", "fail"] },
                    "notes": { "type": "string" },
                    "reviewerAgentId": { "type": "string" }
                },
                "required": ["planPath", "taskId", "reviewType", "result"]
            }),
        },
        ToolSpec {
            name: "orchestrator_accept_task",
            description: "Accept a task after all steps and review gates pass, and check the top-level todo item.",
            input_schema: json!({
                "type": "object",
                "properties": {
                    "planPath": { "type": "string" },
                    "taskId": { "type": "string" }
                },
                "required": ["planPath", "taskId"]
            }),
        },
        ToolSpec {
            name: "orchestrator_check_doc_drift",
            description: "Check whether routing and architecture documents need synchronization based on changed paths.",
            input_schema: json!({
                "type": "object",
                "properties": {
                    "changedPaths": { "type": "array", "items": { "type": "string" } }
                },
                "required": ["changedPaths"]
            }),
        },
        ToolSpec {
            name: "orchestrator_watchdog_tick",
            description: "List stalled tasks that may require continuation or human attention.",
            input_schema: json!({
                "type": "object",
                "properties": {
                    "planId": { "type": "string" },
                    "olderThanMs": { "type": "number" }
                },
                "required": ["planId"]
            }),
        },
        ToolSpec {
            name: "orchestrator_next_action",
            description: "Derive the next deterministic parent-agent action from the active plan and runtime state.",
            input_schema: json!({
                "type": "object",
                "properties": { "planPath": { "type": "string" } },
                "required": ["planPath"]
            }),
        },
        ToolSpec {
            name: "orchestrator_question_gate",
            description: "Decide whether a user-facing question is allowed or whether the parent should continue without asking.",
            input_schema: json!({
                "type": "object",
                "properties": {
                    "questionCategory": { "type": "string" },
                    "userExplicitlyRequested": { "type": "boolean" },
                    "reason": { "type": "string" }
                },
                "required": ["questionCategory"]
            }),
        },
        ToolSpec {
            name: "orchestrator_assess_subagent_completion",
            description: "Assess whether child output is sufficient for review, repair, acceptance, or more implementation.",
            input_schema: json!({
                "type": "object",
                "properties": {
                    "planPath": { "type": "string" },
                    "taskId": { "type": "string" }
                },
                "required": ["planPath", "taskId"]
            }),
        },
        ToolSpec {
            name: "orchestrator_completion_guard",
            description: "Fail closed when the parent tries to end work before plan completion reaches 100 percent.",
            input_schema: json!({
                "type": "object",
                "properties": { "planPath": { "type": "string" } },
                "required": ["planPath"]
            }),
        },
    ]
}

pub fn handle_tool_call(ctx: &AppContext, name: &str, args: &Map<String, Value>) -> Result<Value> {
    match name {
        "orchestrator_resolve_category" => {
            let resolution = ctx.categories.resolve(
                &require_string(args, "title")?,
                &require_string(args, "description")?,
                optional_string(args, "explicitCategory").as_deref(),
            )?;
            tool_result(json!({
                "category_id": resolution.category_id,
                "reason": resolution.reason,
                "preferred_role": resolution.category.preferred_role,
                "allowed_roles": resolution.category.allowed_roles,
                "write_policy": resolution.category.write_policy,
                "requires_plan": resolution.category.requires_plan,
                "requires_spec_review": resolution.category.requires_spec_review,
                "requires_quality_review": resolution.category.requires_quality_review,
                "parallelism": resolution.category.parallelism,
                "delegation_preference": resolution.category.delegation_preference.as_str(),
                "requires_subagent_default": resolution.category.delegation_preference != DelegationPreference::ParentOnly,
                "reuse_policy": resolution.category.reuse_policy,
            }))
        }
        "orchestrator_read_plan_state" => {
            let mut plan = PlanDocument::new(&require_string(args, "planPath")?);
            let plan_id = plan_id_from_path(plan.plan_path())?;
            let payload = plan_state_payload(&plan.read_plan_state()?);
            sync_stored_plan_path(&ctx.runtime_store, &plan_id, plan.plan_path())?;
            tool_result(payload)
        }
        "orchestrator_export_codex_todo" => {
            let mut plan = PlanDocument::new(&require_string(args, "planPath")?);
            let plan_id = plan_id_from_path(plan.plan_path())?;
            sync_stored_plan_path(&ctx.runtime_store, &plan_id, plan.plan_path())?;
            let mirror = build_codex_todo_mirror(&mut plan, &plan_id, &ctx.runtime_store)?;
            sync_stored_plan_path(&ctx.runtime_store, &plan_id, plan.plan_path())?;
            tool_result(json!({
                "plan_id": plan_id,
                "plan_path": plan.plan_path(),
                "items": mirror.items,
                "active_task_id": mirror.active_task_id,
                "active_task_title": mirror.active_task_title,
                "current_step_label": mirror.current_step_label,
                "current_step_text": mirror.current_step_text,
                "remaining_step_count": mirror.remaining_step_count,
                "step_sync_status": mirror.step_sync_status,
                "step_sync_action": mirror.step_sync_action,
                "open_acceptance_items": mirror.open_acceptance_items,
            }))
        }
        "orchestrator_begin_task" => {
            let mut plan = PlanDocument::new(&require_string(args, "planPath")?);
            let task_id = require_string(args, "taskId")?;
            let category_id = require_string(args, "categoryId")?;
            let role = require_string(args, "role")?;
            let assigned_agent = optional_string(args, "assignedAgent");
            let current_wave = optional_string(args, "currentWave");
            let plan_id = plan_id_from_path(plan.plan_path())?;
            sync_stored_plan_path(&ctx.runtime_store, &plan_id, plan.plan_path())?;
            let category = ctx
                .categories
                .get(&category_id)
                .cloned()
                .ok_or_else(|| anyhow!("Unknown category: {category_id}"))?;
            let next_status = optional_string(args, "taskStatus").unwrap_or_else(|| {
                if category_id == "review" {
                    "running_quality_review".to_string()
                } else {
                    "running_impl".to_string()
                }
            });
            if category.write_policy == "lease-required" && next_status == "running_impl" {
                if ctx
                    .runtime_store
                    .get_active_write_lease(&plan_id, &task_id)?
                    .is_none()
                {
                    return Err(anyhow!(
                        "Cannot start {task_id}: write lease required for category {category_id}"
                    ));
                }
            }
            ctx.runtime_store.upsert_plan_state(PlanStateUpsertInput {
                plan_id: plan_id.clone(),
                plan_path: plan.plan_path().to_string(),
                spec_path: None,
                current_wave: current_wave.clone(),
                active_task_id: Some(task_id.clone()),
                last_review_result: None,
            })?;
            ctx.runtime_store.upsert_task_state(TaskStateUpsertInput {
                plan_id: plan_id.clone(),
                task_id: task_id.clone(),
                category_id,
                status: next_status.clone(),
                active_step_label: None,
                assigned_role: Some(role),
                agent_id: assigned_agent.clone(),
                implementation_agent_id: implementation_agent_for_status(&next_status, assigned_agent.clone()),
                review_agent_id: review_agent_for_status(&next_status, assigned_agent.clone()),
                write_lease_id: None,
                spec_review_status: "pending".to_string(),
                quality_review_status: "pending".to_string(),
                retry_count: 0,
                blocker_type: None,
                blocker_message: None,
            })?;
            let current = plan.read_plan_state()?.execution_status;
            plan.update_execution_status(
                Some(current_wave.as_deref().unwrap_or(current.current_wave.as_str())),
                Some(task_id.as_str()),
                Some("None"),
                None,
            )?;
            plan.update_task_metadata(
                &task_id,
                Some(next_status.as_str()),
                None,
                None,
                None,
                Some(assigned_agent.as_deref().unwrap_or("local-parent")),
            )?;
            let mut task = require_plan_task(&plan.read_plan_state()?, &task_id)?;
            let mut step_progress =
                build_step_progress(&task, ctx.runtime_store.get_task_state(&plan_id, &task_id)?);
            if status_uses_step_pointer(&next_status)
                && step_progress.step_sync_status != "all_steps_complete"
                && step_progress.step_sync_status != "step_in_progress"
                && step_progress.next_step_label.is_some()
            {
                sync_task_current_step(
                    &mut plan,
                    &ctx.runtime_store,
                    &plan_id,
                    &task_id,
                    step_progress.next_step_label.as_deref(),
                )?;
                task = require_plan_task(&plan.read_plan_state()?, &task_id)?;
                step_progress =
                    build_step_progress(&task, ctx.runtime_store.get_task_state(&plan_id, &task_id)?);
            }
            tool_result(json!({
                "plan_id": plan_id,
                "task_id": task_id,
                "task_status": next_status,
                "current_step_label": step_progress.current_step_label,
                "current_step_text": step_progress.current_step_text,
                "next_step_label": step_progress.next_step_label,
                "next_step_text": step_progress.next_step_text,
                "remaining_step_count": step_progress.remaining_step_count,
                "step_sync_status": step_progress.step_sync_status,
                "step_sync_action": step_progress.step_sync_action,
            }))
        }
        "orchestrator_acquire_write_lease" => {
            let plan_path = require_string(args, "planPath")?;
            let task_id = require_string(args, "taskId")?;
            let holder_agent_id = require_string(args, "holderAgentId")?;
            let scope = require_string_array(args, "scope")?;
            let plan_id = plan_id_from_path(&plan_path)?;
            let lease = ctx
                .runtime_store
                .acquire_write_lease(&plan_id, &task_id, &holder_agent_id, &scope)?;
            if let Some(current) = ctx.runtime_store.get_task_state(&plan_id, &task_id)? {
                let mut next = task_upsert_from_current(&current);
                next.write_lease_id = Some(lease.lease_id.clone());
                next.blocker_type = None;
                next.blocker_message = None;
                ctx.runtime_store.upsert_task_state(next)?;
            }
            tool_result(json!({
                "plan_id": plan_id,
                "task_id": task_id,
                "lease_id": lease.lease_id,
                "holder_agent_id": lease.holder_agent_id,
                "scope": scope,
                "status": lease.status,
            }))
        }
        "orchestrator_release_write_lease" => {
            let plan_path = require_string(args, "planPath")?;
            let task_id = require_string(args, "taskId")?;
            let lease_id = require_string(args, "leaseId")?;
            let plan_id = plan_id_from_path(&plan_path)?;
            let lease = ctx.runtime_store.release_write_lease(&lease_id)?;
            if let Some(current) = ctx.runtime_store.get_task_state(&plan_id, &task_id)? {
                let mut next = task_upsert_from_current(&current);
                next.write_lease_id = None;
                ctx.runtime_store.upsert_task_state(next)?;
            }
            tool_result(json!({
                "plan_id": plan_id,
                "task_id": task_id,
                "lease_id": lease.lease_id,
                "status": lease.status,
                "released_at": lease.released_at,
            }))
        }
        "orchestrator_begin_step" => {
            let mut plan = PlanDocument::new(&require_string(args, "planPath")?);
            let task_id = require_string(args, "taskId")?;
            let step_label = require_string(args, "stepLabel")?;
            let plan_id = plan_id_from_path(plan.plan_path())?;
            sync_stored_plan_path(&ctx.runtime_store, &plan_id, plan.plan_path())?;
            let existing = ctx
                .runtime_store
                .get_task_state(&plan_id, &task_id)?
                .ok_or_else(|| anyhow!("Task state missing: {task_id}"))?;
            let mut next = task_upsert_from_current(&existing);
            next.active_step_label = Some(step_label.clone());
            ctx.runtime_store.upsert_task_state(next)?;
            plan.update_task_metadata(&task_id, None, Some(&step_label), None, None, None)?;
            let task = require_plan_task(&plan.read_plan_state()?, &task_id)?;
            let step_progress =
                build_step_progress(&task, ctx.runtime_store.get_task_state(&plan_id, &task_id)?);
            tool_result(json!({
                "plan_id": plan_id,
                "task_id": task_id,
                "current_step": step_label,
                "current_step_label": step_progress.current_step_label,
                "current_step_text": step_progress.current_step_text,
                "next_step_label": step_progress.next_step_label,
                "next_step_text": step_progress.next_step_text,
                "remaining_step_count": step_progress.remaining_step_count,
                "step_sync_status": step_progress.step_sync_status,
                "step_sync_action": step_progress.step_sync_action,
            }))
        }
        "orchestrator_complete_step" => {
            let mut plan = PlanDocument::new(&require_string(args, "planPath")?);
            let task_id = require_string(args, "taskId")?;
            let step_label = require_string(args, "stepLabel")?;
            let evidence_summary = optional_string(args, "evidenceSummary");
            let plan_id = plan_id_from_path(plan.plan_path())?;
            sync_stored_plan_path(&ctx.runtime_store, &plan_id, plan.plan_path())?;
            plan.mark_step(&task_id, &step_label, true)?;
            let task_after_check = require_plan_task(&plan.read_plan_state()?, &task_id)?;
            let next_unchecked_step = first_unchecked_step(&task_after_check);
            sync_task_current_step(
                &mut plan,
                &ctx.runtime_store,
                &plan_id,
                &task_id,
                next_unchecked_step.as_ref().map(|step| step.label.as_str()),
            )?;
            if let Some(summary) = evidence_summary.as_deref() {
                ctx.runtime_store.record_evidence(
                    &plan_id,
                    &task_id,
                    "step-completion",
                    None,
                    None,
                    Some(summary),
                )?;
            }
            let synced_task = require_plan_task(&plan.read_plan_state()?, &task_id)?;
            let step_progress =
                build_step_progress(&synced_task, ctx.runtime_store.get_task_state(&plan_id, &task_id)?);
            tool_result(json!({
                "plan_id": plan_id,
                "task_id": task_id,
                "step_label": step_label,
                "checked": true,
                "auto_advanced": next_unchecked_step.is_some(),
                "current_step_label": step_progress.current_step_label,
                "current_step_text": step_progress.current_step_text,
                "next_step_label": step_progress.next_step_label,
                "next_step_text": step_progress.next_step_text,
                "remaining_step_count": step_progress.remaining_step_count,
                "step_sync_status": step_progress.step_sync_status,
                "step_sync_action": step_progress.step_sync_action,
            }))
        }
        "orchestrator_record_subagent_run" => {
            let mut plan = PlanDocument::new(&require_string(args, "planPath")?);
            let task_id = require_string(args, "taskId")?;
            let category_id = require_string(args, "categoryId")?;
            let role = require_string(args, "role")?;
            let agent_id = require_string(args, "agentId")?;
            let status = require_string(args, "status")?;
            let summary = optional_string(args, "summary");
            let plan_id = plan_id_from_path(plan.plan_path())?;
            sync_stored_plan_path(&ctx.runtime_store, &plan_id, plan.plan_path())?;
            let current = ctx.runtime_store.get_task_state(&plan_id, &task_id)?;
            ctx.runtime_store.record_task_run(
                &plan_id,
                &task_id,
                &role,
                &agent_id,
                &status,
                summary.as_deref(),
            )?;
            ctx.runtime_store.upsert_task_state(TaskStateUpsertInput {
                plan_id: plan_id.clone(),
                task_id: task_id.clone(),
                category_id,
                status: current
                    .as_ref()
                    .map(|entry| entry.status.clone())
                    .unwrap_or_else(|| "running_impl".to_string()),
                active_step_label: current.as_ref().and_then(|entry| entry.active_step_label.clone()),
                assigned_role: Some(role),
                agent_id: Some(agent_id.clone()),
                implementation_agent_id: derive_implementation_agent_owner(current.as_ref(), &agent_id),
                review_agent_id: derive_review_agent_owner(current.as_ref(), &agent_id),
                write_lease_id: current.as_ref().and_then(|entry| entry.write_lease_id.clone()),
                spec_review_status: current
                    .as_ref()
                    .map(|entry| entry.spec_review_status.clone())
                    .unwrap_or_else(|| "pending".to_string()),
                quality_review_status: current
                    .as_ref()
                    .map(|entry| entry.quality_review_status.clone())
                    .unwrap_or_else(|| "pending".to_string()),
                retry_count: current.as_ref().map(|entry| entry.retry_count).unwrap_or(0),
                blocker_type: current.as_ref().and_then(|entry| entry.blocker_type.clone()),
                blocker_message: current.as_ref().and_then(|entry| entry.blocker_message.clone()),
            })?;
            plan.update_task_metadata(&task_id, None, None, None, None, Some(&agent_id))?;
            tool_result(json!({
                "plan_id": plan_id,
                "task_id": task_id,
                "agent_id": agent_id,
                "recorded": true,
            }))
        }
        "orchestrator_record_review" => {
            let mut plan = PlanDocument::new(&require_string(args, "planPath")?);
            let task_id = require_string(args, "taskId")?;
            let review_type = require_string(args, "reviewType")?;
            let review_result = require_string(args, "result")?;
            let notes = optional_string(args, "notes");
            let reviewer_agent_id = optional_string(args, "reviewerAgentId");
            if review_type != "spec" && review_type != "quality" {
                return Err(anyhow!("reviewType must be 'spec' or 'quality'"));
            }
            if review_result != "pass" && review_result != "fail" {
                return Err(anyhow!("result must be 'pass' or 'fail'"));
            }
            let plan_id = plan_id_from_path(plan.plan_path())?;
            sync_stored_plan_path(&ctx.runtime_store, &plan_id, plan.plan_path())?;
            let current = ctx
                .runtime_store
                .get_task_state(&plan_id, &task_id)?
                .ok_or_else(|| anyhow!("Task state missing: {task_id}"))?;
            if reviewer_agent_id.as_deref() == current.implementation_agent_id.as_deref()
                && reviewer_agent_id.is_some()
            {
                return Err(anyhow!("Reviewer must not reuse the implementer agent_id"));
            }
            let spec_review_status = if review_type == "spec" {
                review_result.clone()
            } else {
                current.spec_review_status.clone()
            };
            let quality_review_status = if review_type == "quality" {
                review_result.clone()
            } else {
                current.quality_review_status.clone()
            };
            let next_task_status = if review_type == "spec" {
                if review_result == "pass" {
                    "running_quality_review".to_string()
                } else {
                    "spec_failed".to_string()
                }
            } else if review_result == "pass" {
                "impl_done".to_string()
            } else {
                "quality_failed".to_string()
            };
            let mut next = task_upsert_from_current(&current);
            next.status = next_task_status.clone();
            if reviewer_agent_id.is_some() {
                next.agent_id = reviewer_agent_id.clone();
                next.review_agent_id = reviewer_agent_id.clone();
            }
            next.spec_review_status = spec_review_status.clone();
            next.quality_review_status = quality_review_status.clone();
            ctx.runtime_store.upsert_task_state(next)?;
            if let Some(summary) = notes.as_deref() {
                ctx.runtime_store.record_evidence(
                    &plan_id,
                    &task_id,
                    &format!("{review_type}-review"),
                    None,
                    None,
                    Some(summary),
                )?;
            }
            plan.update_task_metadata(
                &task_id,
                Some(&next_task_status),
                None,
                Some(&spec_review_status),
                Some(&quality_review_status),
                None,
            )?;
            let blockers = if review_result == "fail" {
                format!("{task_id} {review_type} review failed")
            } else {
                "None".to_string()
            };
            let review_line = format!("{task_id} {review_type} {review_result}");
            plan.update_execution_status(None, None, Some(&blockers), Some(&review_line))?;
            let acceptance = if review_result == "pass"
                && spec_review_status == "pass"
                && quality_review_status == "pass"
                && plan.all_steps_completed(&task_id)?
            {
                Some(accept_task_in_control_plane(
                    &mut plan,
                    &ctx.runtime_store,
                    &plan_id,
                    &task_id,
                )?)
            } else {
                None
            };
            tool_result(json!({
                "plan_id": plan_id,
                "task_id": task_id,
                "review_type": review_type,
                "result": review_result,
                "task_status": if acceptance.is_some() { "accepted".to_string() } else { next_task_status },
                "accepted": acceptance.is_some(),
                "top_level_todo_checked": acceptance.is_some(),
                "next_active_task_id": acceptance.and_then(|entry| entry.next_active_task_id),
            }))
        }
        "orchestrator_accept_task" => {
            let mut plan = PlanDocument::new(&require_string(args, "planPath")?);
            let task_id = require_string(args, "taskId")?;
            let plan_id = plan_id_from_path(plan.plan_path())?;
            sync_stored_plan_path(&ctx.runtime_store, &plan_id, plan.plan_path())?;
            let acceptance = accept_task_in_control_plane(
                &mut plan,
                &ctx.runtime_store,
                &plan_id,
                &task_id,
            )?;
            tool_result(json!({
                "plan_id": plan_id,
                "task_id": task_id,
                "accepted": true,
                "top_level_todo_checked": true,
                "next_active_task_id": acceptance.next_active_task_id,
            }))
        }
        "orchestrator_check_doc_drift" => {
            tool_result(serde_json::to_value(check_doc_drift(&require_string_array(args, "changedPaths")?))?)
        }
        "orchestrator_watchdog_tick" => {
            let plan_id = require_string(args, "planId")?;
            let older_than_ms = optional_number(args, "olderThanMs").unwrap_or(15 * 60 * 1000);
            let stalled = ctx
                .runtime_store
                .list_stalled_tasks(older_than_ms)?
                .into_iter()
                .filter(|task| task.plan_id == plan_id)
                .collect::<Vec<_>>();
            let mut plan = ctx
                .runtime_store
                .get_plan_state(&plan_id)?
                .map(|record| PlanDocument::new(&record.plan_path));
            let plan_tasks = if let Some(plan) = plan.as_mut() {
                let state = plan.read_plan_state()?;
                sync_stored_plan_path(&ctx.runtime_store, &plan_id, plan.plan_path())?;
                state.tasks
            } else {
                Vec::new()
            };
            tool_result(json!({
                "plan_id": plan_id,
                "stalled_tasks": stalled.into_iter().map(|task| {
                    let plan_task = plan_tasks.iter().find(|entry| entry.id == task.task_id);
                    json!({
                        "task_id": task.task_id,
                        "status": task.status,
                        "active_step": task.active_step_label,
                        "agent_id": task.agent_id,
                        "suggested_action": derive_suggested_action(
                            &task,
                            plan_task,
                            ctx.categories.get(&task.category_id),
                            task.write_lease_id.is_some()
                                || ctx.runtime_store
                                    .get_active_write_lease(&task.plan_id, &task.task_id)
                                    .ok()
                                    .flatten()
                                    .is_some(),
                        ),
                    })
                }).collect::<Vec<_>>()
            }))
        }
        "orchestrator_next_action" => {
            let mut plan = PlanDocument::new(&require_string(args, "planPath")?);
            let plan_id = plan_id_from_path(plan.plan_path())?;
            sync_stored_plan_path(&ctx.runtime_store, &plan_id, plan.plan_path())?;
            let plan_state = plan.read_plan_state()?;
            sync_stored_plan_path(&ctx.runtime_store, &plan_id, plan.plan_path())?;
            let open_acceptance = plan.unchecked_final_acceptance_items()?;
            let next_task = first_dependency_ready_open_task(&plan_state);
            if let Some(next_task) = next_task {
                let task_state = ctx.runtime_store.get_task_state(&plan_id, &next_task.id)?;
                let active_lease = ctx.runtime_store.get_active_write_lease(&plan_id, &next_task.id)?;
                let mut action = derive_next_action(
                    &next_task,
                    task_state.as_ref(),
                    ctx.categories.get(&next_task.category),
                    active_lease.is_some(),
                    &open_acceptance,
                );
                action = decorate_action_with_task_session(&plan_id, &next_task, task_state.as_ref(), action);
                let parallel_batch = build_parallel_dispatch_batch(
                    &plan_state,
                    &ctx.runtime_store,
                    &ctx.categories,
                    &plan_id,
                    &next_task,
                    &action,
                    &open_acceptance,
                )?;
                let parallel_dispatches = parallel_batch
                    .as_ref()
                    .map(|batch| batch.entries.clone())
                    .unwrap_or_default();
                if let Some(batch) = parallel_batch.filter(|batch| batch.entries.len() > 1) {
                    action.action = batch.top_level_action;
                    action.reason = format!(
                        "Tasks {} are dependency-ready, conflict-free, and can be advanced together.",
                        batch.entries
                            .iter()
                            .map(|entry| entry.task_id.as_str())
                            .collect::<Vec<_>>()
                            .join(", ")
                    );
                }
                return tool_result(shape_next_action_payload(
                    &plan_id,
                    &action,
                    Some(&next_task.id),
                    &parallel_dispatches,
                ));
            }
            let open_tasks = plan_state
                .tasks
                .iter()
                .filter(|task| !task.todo_checked)
                .collect::<Vec<_>>();
            if !open_tasks.is_empty() {
                let blocked_task = open_tasks[0];
                let action = with_delegation_metadata(
                    None,
                    RawActionState {
                        task_id: Some(blocked_task.id.clone()),
                        open_acceptance_items: Vec::new(),
                        action: "wait_for_dependencies".to_string(),
                        category_id: Some(blocked_task.category.clone()),
                        required_role: None,
                        requires_write_lease: false,
                        reason: format!(
                            "Task {} is still waiting on unresolved dependencies: {}.",
                            blocked_task.id,
                            blocked_task.depends_on.join(", ")
                        ),
                        step_progress: empty_step_progress(),
                    },
                );
                return tool_result(shape_next_action_payload(&plan_id, &action, Some(&blocked_task.id), &[]));
            }
            if !open_acceptance.is_empty() {
                let action = with_delegation_metadata(
                    None,
                    RawActionState {
                        task_id: None,
                        open_acceptance_items: open_acceptance.clone(),
                        action: "complete_final_acceptance".to_string(),
                        category_id: None,
                        required_role: None,
                        requires_write_lease: false,
                        reason: format!(
                            "All top-level tasks are accepted, but final acceptance still has open items: {}.",
                            open_acceptance.join(", ")
                        ),
                        step_progress: empty_step_progress(),
                    },
                );
                return tool_result(shape_next_action_payload(&plan_id, &action, None, &[]));
            }
            let action = with_delegation_metadata(
                None,
                RawActionState {
                    task_id: None,
                    open_acceptance_items: Vec::new(),
                    action: "complete_plan".to_string(),
                    category_id: None,
                    required_role: None,
                    requires_write_lease: false,
                    reason: "All top-level tasks are already accepted.".to_string(),
                    step_progress: empty_step_progress(),
                },
            );
            tool_result(shape_next_action_payload(&plan_id, &action, None, &[]))
        }
        "orchestrator_question_gate" => {
            let question_category = require_string(args, "questionCategory")?;
            let user_explicitly_requested = optional_bool(args, "userExplicitlyRequested").unwrap_or(false);
            let reason = optional_string(args, "reason").unwrap_or_default();
            let hard_blockers: HashSet<&str> =
                ["identity", "credential", "destructive", "conflict"]
                    .into_iter()
                    .collect();
            if question_category == "direction_confirmation" {
                return tool_result(json!({
                    "ask_user": false,
                    "blocker_type": "none",
                    "allowed_to_expand": false,
                    "recommended_action": "plan_and_execute",
                    "reason": if reason.is_empty() {
                        "The user already supplied a workable direction, so do not ask for a second start confirmation.".to_string()
                    } else {
                        reason
                    }
                }));
            }
            if question_category == "optional_expansion" && !user_explicitly_requested {
                return tool_result(json!({
                    "ask_user": false,
                    "blocker_type": "none",
                    "allowed_to_expand": false,
                    "recommended_action": "skip_optional_expansion",
                    "reason": if reason.is_empty() {
                        "Optional expansion was not explicitly requested by the user.".to_string()
                    } else {
                        reason
                    }
                }));
            }
            if hard_blockers.contains(question_category.as_str()) {
                return tool_result(json!({
                    "ask_user": true,
                    "blocker_type": question_category,
                    "allowed_to_expand": false,
                    "recommended_action": "ask_user",
                    "reason": if reason.is_empty() {
                        format!("A hard blocker of type {} requires user resolution.", question_category)
                    } else {
                        reason
                    }
                }));
            }
            if question_category == "system" {
                return tool_result(json!({
                    "ask_user": false,
                    "blocker_type": "system",
                    "allowed_to_expand": false,
                    "recommended_action": "retry_or_report",
                    "reason": if reason.is_empty() {
                        "System-level issues should be retried or reported instead of turned into user questions.".to_string()
                    } else {
                        reason
                    }
                }));
            }
            tool_result(json!({
                "ask_user": false,
                "blocker_type": "none",
                "allowed_to_expand": user_explicitly_requested,
                "recommended_action": if user_explicitly_requested {
                    "execute_requested_scope"
                } else {
                    "record_assumption_and_continue"
                },
                "reason": if reason.is_empty() {
                    "No hard blocker requires a user-facing question.".to_string()
                } else {
                    reason
                }
            }))
        }
        "orchestrator_assess_subagent_completion" => {
            let mut plan = PlanDocument::new(&require_string(args, "planPath")?);
            let task_id = require_string(args, "taskId")?;
            let plan_id = plan_id_from_path(plan.plan_path())?;
            sync_stored_plan_path(&ctx.runtime_store, &plan_id, plan.plan_path())?;
            let assessment =
                assess_subagent_completion(&mut plan, &plan_id, &task_id, &ctx.runtime_store)?;
            tool_result(json!({
                "task_id": task_id,
                "implementation_complete": assessment.implementation_complete,
                "missing_steps": assessment.missing_steps,
                "missing_evidence": assessment.missing_evidence,
                "next_required_stage": assessment.next_required_stage,
                "repair_role": assessment.repair_role,
                "can_accept": assessment.can_accept,
            }))
        }
        "orchestrator_completion_guard" => {
            let mut plan = PlanDocument::new(&require_string(args, "planPath")?);
            let plan_id = plan_id_from_path(plan.plan_path())?;
            sync_stored_plan_path(&ctx.runtime_store, &plan_id, plan.plan_path())?;
            let plan_state = plan.read_plan_state()?;
            sync_stored_plan_path(&ctx.runtime_store, &plan_id, plan.plan_path())?;
            let open_tasks = plan_state
                .tasks
                .into_iter()
                .filter(|task| !task.todo_checked)
                .map(|task| task.id)
                .collect::<Vec<_>>();
            let open_acceptance_items = plan.unchecked_final_acceptance_items()?;
            let can_finish = open_tasks.is_empty() && open_acceptance_items.is_empty();
            let mut blockers = open_tasks.clone();
            blockers.extend(open_acceptance_items.clone());
            tool_result(json!({
                "can_finish": can_finish,
                "open_tasks": open_tasks,
                "open_acceptance_items": open_acceptance_items,
                "blocking_reason": if can_finish {
                    "Plan completion is at 100 percent.".to_string()
                } else {
                    format!("Open tasks or final acceptance items remain: {}", blockers.join(", "))
                }
            }))
        }
        other => Err(anyhow!("Unknown tool: {other}")),
    }
}

fn tool_result(payload: Value) -> Result<Value> {
    Ok(json!({
        "content": [{ "type": "text", "text": serde_json::to_string_pretty(&payload)? }],
        "structuredContent": payload,
    }))
}

fn plan_id_from_path(plan_path: &str) -> Result<String> {
    Path::new(plan_path)
        .file_stem()
        .and_then(|value| value.to_str())
        .map(|value| value.to_string())
        .ok_or_else(|| anyhow!("Unable to derive plan id from path: {plan_path}"))
}

fn sync_stored_plan_path(runtime_store: &RuntimeStore, plan_id: &str, plan_path: &str) -> Result<()> {
    if let Some(existing) = runtime_store.get_plan_state(plan_id)? {
        if existing.plan_path != plan_path {
            runtime_store.upsert_plan_state(PlanStateUpsertInput {
                plan_id: plan_id.to_string(),
                plan_path: plan_path.to_string(),
                spec_path: existing.spec_path,
                current_wave: existing.current_wave,
                active_task_id: existing.active_task_id,
                last_review_result: existing.last_review_result,
            })?;
        }
    }
    Ok(())
}

fn require_string(args: &Map<String, Value>, key: &str) -> Result<String> {
    args.get(key)
        .and_then(Value::as_str)
        .map(|value| value.to_string())
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| anyhow!("Expected string argument: {key}"))
}

fn optional_string(args: &Map<String, Value>, key: &str) -> Option<String> {
    args.get(key).and_then(Value::as_str).map(|value| value.to_string())
}

fn optional_number(args: &Map<String, Value>, key: &str) -> Option<i64> {
    args.get(key).and_then(Value::as_i64)
}

fn optional_bool(args: &Map<String, Value>, key: &str) -> Option<bool> {
    args.get(key).and_then(Value::as_bool)
}

fn require_string_array(args: &Map<String, Value>, key: &str) -> Result<Vec<String>> {
    args.get(key)
        .and_then(Value::as_array)
        .ok_or_else(|| anyhow!("Expected string[] argument: {key}"))?
        .iter()
        .map(|entry| {
            entry
                .as_str()
                .map(|value| value.to_string())
                .ok_or_else(|| anyhow!("Expected string[] argument: {key}"))
        })
        .collect()
}

fn empty_step_progress() -> StepProgressState {
    StepProgressState {
        current_step_label: None,
        current_step_text: None,
        next_step_label: None,
        next_step_text: None,
        remaining_step_count: 0,
        step_sync_status: "all_steps_complete".to_string(),
        step_sync_action: "none".to_string(),
    }
}

fn resolve_codex_todo_active_task(plan_tasks: &[PlanTask], active_task_id: Option<&str>) -> Option<PlanTask> {
    let open_tasks = plan_tasks
        .iter()
        .filter(|task| !task.todo_checked)
        .cloned()
        .collect::<Vec<_>>();
    if open_tasks.is_empty() {
        return None;
    }
    if let Some(active_task_id) = active_task_id.filter(|value| *value != "none") {
        if let Some(task) = open_tasks.iter().find(|task| task.id == active_task_id) {
            return Some(task.clone());
        }
    }
    open_tasks.first().cloned()
}

fn format_codex_todo_step(plan_task: &PlanTask, status: &str, step_progress: Option<&StepProgressState>) -> String {
    let base = format!("{}. {}", plan_task.id, plan_task.title);
    if status != "in_progress" {
        return base;
    }
    let Some(step_progress) = step_progress else {
        return base;
    };
    let step_label = step_progress
        .current_step_label
        .as_ref()
        .or(step_progress.next_step_label.as_ref());
    let step_text = step_progress
        .current_step_text
        .as_ref()
        .or(step_progress.next_step_text.as_ref());
    match (step_label, step_text) {
        (Some(step_label), Some(step_text)) => format!("{base} ({step_label}: {step_text})"),
        _ => base,
    }
}

fn format_final_acceptance_step(open_acceptance_items: &[String]) -> String {
    if open_acceptance_items.len() == 1 {
        format!("Final acceptance ({})", open_acceptance_items[0])
    } else {
        format!(
            "Final acceptance ({} items remaining)",
            open_acceptance_items.len()
        )
    }
}

fn build_codex_todo_mirror(
    plan: &mut PlanDocument,
    plan_id: &str,
    runtime_store: &RuntimeStore,
) -> Result<CodexTodoMirror> {
    let plan_state = plan.read_plan_state()?;
    let open_acceptance_items = plan.unchecked_final_acceptance_items()?;
    let active_task = resolve_codex_todo_active_task(
        &plan_state.tasks,
        Some(plan_state.execution_status.active_task.as_str()),
    );
    let active_task_state = if let Some(active_task) = active_task.as_ref() {
        runtime_store.get_task_state(plan_id, &active_task.id)?
    } else {
        None
    };
    let step_progress = if let Some(active_task) = active_task.as_ref() {
        build_step_progress(active_task, active_task_state)
    } else {
        empty_step_progress()
    };
    let mut items = Vec::new();
    for task in &plan_state.tasks {
        let status = if task.todo_checked {
            "completed"
        } else if active_task.as_ref().map(|entry| entry.id.as_str()) == Some(task.id.as_str()) {
            "in_progress"
        } else {
            "pending"
        };
        items.push(CodexTodoItem {
            step: format_codex_todo_step(
                task,
                status,
                if active_task.as_ref().map(|entry| entry.id.as_str()) == Some(task.id.as_str()) {
                    Some(&step_progress)
                } else {
                    None
                },
            ),
            status: status.to_string(),
        });
    }

    if active_task.is_none() && !open_acceptance_items.is_empty() {
        items.push(CodexTodoItem {
            step: format_final_acceptance_step(&open_acceptance_items),
            status: "in_progress".to_string(),
        });
    }

    Ok(CodexTodoMirror {
        items,
        active_task_id: active_task.as_ref().map(|task| task.id.clone()),
        active_task_title: active_task.as_ref().map(|task| task.title.clone()),
        current_step_label: step_progress.current_step_label,
        current_step_text: step_progress.current_step_text,
        remaining_step_count: step_progress.remaining_step_count,
        step_sync_status: step_progress.step_sync_status,
        step_sync_action: step_progress.step_sync_action,
        open_acceptance_items,
    })
}

fn first_unchecked_step(plan_task: &PlanTask) -> Option<PlanStep> {
    plan_task.steps.iter().find(|step| !step.checked).cloned()
}

fn find_step_by_label(plan_task: &PlanTask, label: Option<&str>) -> Option<PlanStep> {
    label
        .filter(|value| *value != "none")
        .and_then(|label| plan_task.steps.iter().find(|step| step.label == label).cloned())
}

fn build_step_progress(plan_task: &PlanTask, task_state: Option<TaskStateRecord>) -> StepProgressState {
    let next_step = first_unchecked_step(plan_task);
    let remaining_step_count = plan_task.steps.iter().filter(|step| !step.checked).count();
    let Some(next_step) = next_step else {
        return empty_step_progress();
    };

    let declared_current_step_label = task_state
        .as_ref()
        .and_then(|entry| entry.active_step_label.clone())
        .or_else(|| {
            if plan_task.current_step != "none" {
                Some(plan_task.current_step.clone())
            } else {
                None
            }
        });
    let current_step = find_step_by_label(plan_task, declared_current_step_label.as_deref());

    if declared_current_step_label.is_none() {
        return StepProgressState {
            current_step_label: None,
            current_step_text: None,
            next_step_label: Some(next_step.label),
            next_step_text: Some(next_step.text),
            remaining_step_count,
            step_sync_status: "needs_begin_step".to_string(),
            step_sync_action: "begin_next_step".to_string(),
        };
    }

    if current_step
        .as_ref()
        .map(|step| step.checked || step.label != next_step.label)
        .unwrap_or(true)
    {
        return StepProgressState {
            current_step_label: declared_current_step_label,
            current_step_text: None,
            next_step_label: Some(next_step.label),
            next_step_text: Some(next_step.text),
            remaining_step_count,
            step_sync_status: "stale_current_step".to_string(),
            step_sync_action: "repair_current_step".to_string(),
        };
    }

    let current_step = current_step.expect("checked above");
    StepProgressState {
        current_step_label: Some(current_step.label.clone()),
        current_step_text: Some(current_step.text.clone()),
        next_step_label: Some(next_step.label),
        next_step_text: Some(next_step.text),
        remaining_step_count,
        step_sync_status: "step_in_progress".to_string(),
        step_sync_action: "continue_current_step".to_string(),
    }
}

fn status_uses_step_pointer(status: &str) -> bool {
    matches!(
        status,
        "running_impl" | "running_spec_review" | "running_quality_review"
    )
}

fn sync_task_current_step(
    plan: &mut PlanDocument,
    runtime_store: &RuntimeStore,
    plan_id: &str,
    task_id: &str,
    next_step_label: Option<&str>,
) -> Result<()> {
    let current = runtime_store
        .get_task_state(plan_id, task_id)?
        .ok_or_else(|| anyhow!("Task state missing: {task_id}"))?;
    let mut next = task_upsert_from_current(&current);
    next.active_step_label = next_step_label.map(|value| value.to_string());
    runtime_store.upsert_task_state(next)?;
    plan.update_task_metadata(task_id, None, Some(next_step_label.unwrap_or("none")), None, None, None)?;
    Ok(())
}

fn accept_task_in_control_plane(
    plan: &mut PlanDocument,
    runtime_store: &RuntimeStore,
    plan_id: &str,
    task_id: &str,
) -> Result<TaskAcceptanceOutcome> {
    let current = runtime_store
        .get_task_state(plan_id, task_id)?
        .ok_or_else(|| anyhow!("Task state missing: {task_id}"))?;
    if !plan.all_steps_completed(task_id)? {
        return Err(anyhow!(
            "Cannot accept task {task_id}: plan steps are not all checked"
        ));
    }
    if current.spec_review_status != "pass" || current.quality_review_status != "pass" {
        return Err(anyhow!(
            "Cannot accept task {task_id}: both review gates must pass first"
        ));
    }

    let mut next = task_upsert_from_current(&current);
    next.status = "accepted".to_string();
    next.active_step_label = None;
    next.blocker_type = None;
    next.blocker_message = None;
    runtime_store.upsert_task_state(next)?;
    plan.update_task_metadata(task_id, Some("accepted"), Some("none"), None, None, None)?;
    plan.mark_top_level_todo(task_id, true)?;

    let next_active_task_id = plan
        .read_plan_state()?
        .tasks
        .iter()
        .find(|task| !task.todo_checked)
        .map(|task| task.id.clone());
    plan.update_execution_status(
        None,
        Some(next_active_task_id.as_deref().unwrap_or("none")),
        None,
        None,
    )?;
    if let Some(existing) = runtime_store.get_plan_state(plan_id)? {
        runtime_store.upsert_plan_state(PlanStateUpsertInput {
            plan_id: plan_id.to_string(),
            plan_path: plan.plan_path().to_string(),
            spec_path: existing.spec_path,
            current_wave: existing.current_wave,
            active_task_id: next_active_task_id.clone(),
            last_review_result: existing.last_review_result,
        })?;
    }

    Ok(TaskAcceptanceOutcome { next_active_task_id })
}

fn task_upsert_from_current(current: &TaskStateRecord) -> TaskStateUpsertInput {
    TaskStateUpsertInput {
        plan_id: current.plan_id.clone(),
        task_id: current.task_id.clone(),
        category_id: current.category_id.clone(),
        status: current.status.clone(),
        active_step_label: current.active_step_label.clone(),
        assigned_role: current.assigned_role.clone(),
        agent_id: current.agent_id.clone(),
        implementation_agent_id: current.implementation_agent_id.clone(),
        review_agent_id: current.review_agent_id.clone(),
        write_lease_id: current.write_lease_id.clone(),
        spec_review_status: current.spec_review_status.clone(),
        quality_review_status: current.quality_review_status.clone(),
        retry_count: current.retry_count,
        blocker_type: current.blocker_type.clone(),
        blocker_message: current.blocker_message.clone(),
    }
}

fn implementation_agent_for_status(status: &str, assigned_agent: Option<String>) -> Option<String> {
    if matches!(status, "running_impl" | "impl_done" | "spec_failed" | "quality_failed") {
        return assigned_agent;
    }
    None
}

fn review_agent_for_status(status: &str, assigned_agent: Option<String>) -> Option<String> {
    if matches!(status, "running_spec_review" | "running_quality_review") {
        return assigned_agent;
    }
    None
}

fn derive_implementation_agent_owner(current: Option<&TaskStateRecord>, agent_id: &str) -> Option<String> {
    let current_status = current.map(|entry| entry.status.as_str()).unwrap_or("running_impl");
    if matches!(
        current_status,
        "running_impl" | "impl_done" | "spec_failed" | "quality_failed"
    ) {
        return Some(agent_id.to_string());
    }
    current.and_then(|entry| entry.implementation_agent_id.clone())
}

fn derive_review_agent_owner(current: Option<&TaskStateRecord>, agent_id: &str) -> Option<String> {
    let current_status = current.map(|entry| entry.status.as_str()).unwrap_or("planned");
    if matches!(current_status, "running_spec_review" | "running_quality_review") {
        return Some(agent_id.to_string());
    }
    current.and_then(|entry| entry.review_agent_id.clone())
}

fn derive_suggested_action(
    task_state: &TaskStateRecord,
    plan_task: Option<&PlanTask>,
    category: Option<&CategoryDefinition>,
    active_lease_present: bool,
) -> String {
    if category.map(|entry| entry.write_policy.as_str()) == Some("lease-required")
        && !active_lease_present
    {
        return "acquire_write_lease".to_string();
    }
    if let Some(plan_task) = plan_task {
        if matches!(
            task_state.status.as_str(),
            "running_impl" | "running_spec_review" | "running_quality_review"
        ) {
            let step_progress = build_step_progress(plan_task, Some(task_state.clone()));
            if step_progress.step_sync_status == "needs_begin_step"
                || step_progress.step_sync_status == "stale_current_step"
            {
                return "repair_step_sync".to_string();
            }
        }
    }
    if task_state.status == "spec_failed" || task_state.status == "quality_failed" {
        return "return_to_implementer".to_string();
    }
    if task_state.status == "running_quality_review" || task_state.status == "running_spec_review" {
        return "re-run_review".to_string();
    }
    if task_state.status == "blocked" {
        return "mark_blocked".to_string();
    }
    "continue_same_agent".to_string()
}

fn assess_subagent_completion(
    plan: &mut PlanDocument,
    plan_id: &str,
    task_id: &str,
    runtime_store: &RuntimeStore,
) -> Result<CompletionAssessment> {
    let plan_state = plan.read_plan_state()?;
    let plan_task = require_plan_task(&plan_state, task_id)?;
    let task_state = runtime_store.get_task_state(plan_id, task_id)?;
    let evidence = runtime_store.list_evidence_for_task(plan_id, task_id)?;
    let missing_steps = plan_task
        .steps
        .iter()
        .filter(|step| !step.checked)
        .map(|step| step.label.clone())
        .collect::<Vec<_>>();
    let missing_evidence = evidence.is_empty();
    let implementation_complete = missing_steps.is_empty()
        && task_state
            .as_ref()
            .map(|entry| entry.status.as_str() != "blocked" && entry.status.as_str() != "cancelled")
            .unwrap_or(true);

    let (next_required_stage, repair_role) = if !implementation_complete {
        ("implementation".to_string(), Some(plan_task.owner_role.clone()))
    } else if missing_evidence {
        (
            "implementation_evidence".to_string(),
            Some(plan_task.owner_role.clone()),
        )
    } else if plan_task.spec_review_status != "pass" {
        if plan_task.spec_review_status == "fail" {
            ("repair".to_string(), Some(plan_task.owner_role.clone()))
        } else {
            ("spec_review".to_string(), Some("harness-evaluator".to_string()))
        }
    } else if plan_task.quality_review_status != "pass" {
        if plan_task.quality_review_status == "fail" {
            ("repair".to_string(), Some(plan_task.owner_role.clone()))
        } else {
            ("quality_review".to_string(), Some("harness-evaluator".to_string()))
        }
    } else if plan_task.todo_checked {
        ("done".to_string(), None)
    } else {
        ("accept".to_string(), None)
    };

    let can_accept = implementation_complete
        && !missing_evidence
        && plan_task.spec_review_status == "pass"
        && plan_task.quality_review_status == "pass";

    Ok(CompletionAssessment {
        implementation_complete,
        missing_steps,
        missing_evidence,
        next_required_stage,
        repair_role,
        can_accept,
    })
}

fn derive_next_action(
    plan_task: &PlanTask,
    task_state: Option<&TaskStateRecord>,
    category: Option<&CategoryDefinition>,
    active_lease_present: bool,
    final_acceptance_open: &[String],
) -> ActionState {
    let requires_write_lease = category.map(|entry| entry.write_policy.as_str()) == Some("lease-required");
    let step_progress = build_step_progress(plan_task, task_state.cloned());

    if requires_write_lease && !active_lease_present {
        return with_delegation_metadata(
            category,
            RawActionState {
                task_id: Some(plan_task.id.clone()),
                open_acceptance_items: Vec::new(),
                action: "acquire_write_lease".to_string(),
                category_id: Some(plan_task.category.clone()),
                required_role: Some(plan_task.owner_role.clone()),
                requires_write_lease: true,
                reason: format!(
                    "Task {} belongs to a lease-required category and has no active lease.",
                    plan_task.id
                ),
                step_progress,
            },
        );
    }

    if task_state.map(|entry| entry.status.as_str()) == Some("impl_done")
        && (plan_task.spec_review_status == "pending" || plan_task.spec_review_status == "fail")
    {
        return with_delegation_metadata(
            category,
            RawActionState {
                task_id: Some(plan_task.id.clone()),
                open_acceptance_items: Vec::new(),
                action: if plan_task.spec_review_status == "fail" {
                    "repair_and_re_review".to_string()
                } else {
                    "run_spec_review".to_string()
                },
                category_id: Some(plan_task.category.clone()),
                required_role: Some(if plan_task.spec_review_status == "fail" {
                    plan_task.owner_role.clone()
                } else {
                    "harness-evaluator".to_string()
                }),
                requires_write_lease,
                reason: if plan_task.spec_review_status == "fail" {
                    format!(
                        "Task {} failed spec review and must return to implementation before review repeats.",
                        plan_task.id
                    )
                } else {
                    format!(
                        "Task {} needs spec review before it can continue.",
                        plan_task.id
                    )
                },
                step_progress,
            },
        );
    }

    if task_state.map(|entry| entry.status.as_str()) == Some("impl_done")
        && plan_task.spec_review_status == "pass"
        && (plan_task.quality_review_status == "pending"
            || plan_task.quality_review_status == "fail")
    {
        return with_delegation_metadata(
            category,
            RawActionState {
                task_id: Some(plan_task.id.clone()),
                open_acceptance_items: Vec::new(),
                action: if plan_task.quality_review_status == "fail" {
                    "repair_and_re_review".to_string()
                } else {
                    "run_quality_review".to_string()
                },
                category_id: Some(plan_task.category.clone()),
                required_role: Some(if plan_task.quality_review_status == "fail" {
                    plan_task.owner_role.clone()
                } else {
                    "harness-evaluator".to_string()
                }),
                requires_write_lease,
                reason: if plan_task.quality_review_status == "fail" {
                    format!(
                        "Task {} failed quality review and must return to implementation before review repeats.",
                        plan_task.id
                    )
                } else {
                    format!(
                        "Task {} needs quality review before it can be accepted.",
                        plan_task.id
                    )
                },
                step_progress,
            },
        );
    }

    if matches!(
        task_state.map(|entry| entry.status.as_str()),
        Some("spec_failed") | Some("quality_failed")
    ) {
        return with_delegation_metadata(
            category,
            RawActionState {
                task_id: Some(plan_task.id.clone()),
                open_acceptance_items: Vec::new(),
                action: "return_to_implementer".to_string(),
                category_id: Some(plan_task.category.clone()),
                required_role: Some(
                    task_state
                        .and_then(|entry| entry.assigned_role.clone())
                        .unwrap_or_else(|| plan_task.owner_role.clone()),
                ),
                requires_write_lease,
                reason: format!(
                    "Task {} failed a review gate and must return to implementation.",
                    plan_task.id
                ),
                step_progress,
            },
        );
    }

    if plan_task.spec_review_status == "pass"
        && plan_task.quality_review_status == "pass"
        && plan_task.steps.iter().all(|step| step.checked)
    {
        return with_delegation_metadata(
            category,
            RawActionState {
                task_id: Some(plan_task.id.clone()),
                open_acceptance_items: Vec::new(),
                action: "accept_task".to_string(),
                category_id: Some(plan_task.category.clone()),
                required_role: None,
                requires_write_lease,
                reason: format!(
                    "Task {} has all steps checked and both review gates passed.",
                    plan_task.id
                ),
                step_progress,
            },
        );
    }

    if plan_task.steps.iter().all(|step| step.checked) && plan_task.spec_review_status == "pending" {
        return with_delegation_metadata(
            category,
            RawActionState {
                task_id: Some(plan_task.id.clone()),
                open_acceptance_items: Vec::new(),
                action: "run_spec_review".to_string(),
                category_id: Some(plan_task.category.clone()),
                required_role: Some("harness-evaluator".to_string()),
                requires_write_lease,
                reason: format!(
                    "Task {} has completed implementation steps and now needs spec review.",
                    plan_task.id
                ),
                step_progress,
            },
        );
    }

    if plan_task.steps.iter().all(|step| step.checked)
        && plan_task.spec_review_status == "pass"
        && plan_task.quality_review_status == "pending"
    {
        return with_delegation_metadata(
            category,
            RawActionState {
                task_id: Some(plan_task.id.clone()),
                open_acceptance_items: Vec::new(),
                action: "run_quality_review".to_string(),
                category_id: Some(plan_task.category.clone()),
                required_role: Some("harness-evaluator".to_string()),
                requires_write_lease,
                reason: format!(
                    "Task {} passed spec review and now needs quality review.",
                    plan_task.id
                ),
                step_progress,
            },
        );
    }

    if plan_task.todo_checked && !final_acceptance_open.is_empty() {
        return with_delegation_metadata(
            category,
            RawActionState {
                task_id: Some(plan_task.id.clone()),
                open_acceptance_items: final_acceptance_open.to_vec(),
                action: "complete_final_acceptance".to_string(),
                category_id: Some(plan_task.category.clone()),
                required_role: None,
                requires_write_lease: false,
                reason: format!(
                    "All top-level tasks are accepted, but final acceptance still has open items: {}.",
                    final_acceptance_open.join(", ")
                ),
                step_progress,
            },
        );
    }

    if task_state.map(|entry| entry.status.as_str()) == Some("running_impl") {
        return with_delegation_metadata(
            category,
            RawActionState {
                task_id: Some(plan_task.id.clone()),
                open_acceptance_items: Vec::new(),
                action: "continue_same_agent".to_string(),
                category_id: Some(plan_task.category.clone()),
                required_role: Some(
                    task_state
                        .and_then(|entry| entry.assigned_role.clone())
                        .unwrap_or_else(|| plan_task.owner_role.clone()),
                ),
                requires_write_lease,
                reason: format!("Task {} is already in implementation progress.", plan_task.id),
                step_progress,
            },
        );
    }

    if matches!(
        task_state.map(|entry| entry.status.as_str()),
        Some("running_spec_review") | Some("running_quality_review")
    ) {
        return with_delegation_metadata(
            category,
            RawActionState {
                task_id: Some(plan_task.id.clone()),
                open_acceptance_items: Vec::new(),
                action: "continue_same_agent".to_string(),
                category_id: Some(plan_task.category.clone()),
                required_role: Some(
                    task_state
                        .and_then(|entry| entry.assigned_role.clone())
                        .unwrap_or_else(|| "harness-evaluator".to_string()),
                ),
                requires_write_lease,
                reason: format!(
                    "Task {} already has an in-progress review assignment.",
                    plan_task.id
                ),
                step_progress,
            },
        );
    }

    with_delegation_metadata(
        category,
        RawActionState {
            task_id: Some(plan_task.id.clone()),
            open_acceptance_items: Vec::new(),
            action: "dispatch_task".to_string(),
            category_id: Some(plan_task.category.clone()),
            required_role: Some(plan_task.owner_role.clone()),
            requires_write_lease,
            reason: format!(
                "Task {} is the next incomplete top-level task in the plan.",
                plan_task.id
            ),
            step_progress,
        },
    )
}

fn default_task_status_for_category(category_id: &str) -> String {
    if category_id == "review" {
        "running_quality_review".to_string()
    } else {
        "running_impl".to_string()
    }
}

fn blocking_control_plane_actions(
    category: Option<&CategoryDefinition>,
    action: &RawActionState,
) -> Vec<BlockingControlPlaneAction> {
    let mut actions = Vec::new();
    let Some(task_id) = action.task_id.clone() else {
        return actions;
    };

    if action.required_role.is_none() {
        return actions;
    }

    let category_id = action.category_id.clone();
    let category_id_str = category_id
        .as_deref()
        .or_else(|| category.map(|entry| entry.id.as_str()))
        .unwrap_or("unknown");
    let step_progress = &action.step_progress;

    if action.action == "dispatch_task" && step_progress.current_step_label.is_none() {
        actions.push(BlockingControlPlaneAction {
            action: "begin_task".to_string(),
            tool_name: "orchestrator_begin_task".to_string(),
            reason: format!(
                "Task {} must be entered into the control plane before child execution starts.",
                task_id
            ),
            task_id: task_id.clone(),
            category_id: Some(category_id_str.to_string()),
            role: action.required_role.clone(),
            task_status: Some(default_task_status_for_category(category_id_str)),
            step_label: None,
        });
        return actions;
    }

    if matches!(action.action.as_str(), "dispatch_task" | "continue_same_agent" | "return_to_implementer" | "repair_and_re_review")
    {
        match step_progress.step_sync_status.as_str() {
            "needs_begin_step" => {
                if let Some(step_label) = step_progress.next_step_label.clone() {
                    actions.push(BlockingControlPlaneAction {
                        action: "begin_step".to_string(),
                        tool_name: "orchestrator_begin_step".to_string(),
                        reason: format!(
                            "Task {} needs an active current-step pointer before child execution continues.",
                            task_id
                        ),
                        task_id: task_id.clone(),
                        category_id: Some(category_id_str.to_string()),
                        role: action.required_role.clone(),
                        task_status: None,
                        step_label: Some(step_label),
                    });
                }
            }
            "stale_current_step" => {
                if let Some(step_label) = step_progress.next_step_label.clone() {
                    actions.push(BlockingControlPlaneAction {
                        action: "repair_current_step".to_string(),
                        tool_name: "orchestrator_begin_step".to_string(),
                        reason: format!(
                            "Task {} has stale step sync and must repair the current step pointer before child execution continues.",
                            task_id
                        ),
                        task_id: task_id.clone(),
                        category_id: Some(category_id_str.to_string()),
                        role: action.required_role.clone(),
                        task_status: None,
                        step_label: Some(step_label),
                    });
                }
            }
            _ => {}
        }
    }

    actions
}

fn child_execution_scope(
    action_name: &str,
    step_progress: &StepProgressState,
) -> (String, Option<String>, Option<String>) {
    if matches!(
        action_name,
        "dispatch_task" | "continue_same_agent" | "return_to_implementer" | "repair_and_re_review"
    ) {
        return (
            "current-step".to_string(),
            step_progress
                .current_step_label
                .clone()
                .or_else(|| step_progress.next_step_label.clone()),
            step_progress
                .current_step_text
                .clone()
                .or_else(|| step_progress.next_step_text.clone()),
        );
    }

    if matches!(action_name, "run_spec_review" | "run_quality_review") {
        return ("review-pass".to_string(), None, None);
    }

    ("parent-local".to_string(), None, None)
}

fn subagent_tool_action(session_mode: &str) -> String {
    if session_mode.starts_with("spawn-") {
        "spawn_agent".to_string()
    } else if session_mode.starts_with("resume-") {
        "send_input".to_string()
    } else {
        "none".to_string()
    }
}

fn subagent_dispatch_message(
    task_id: &str,
    task_title: &str,
    category_id: Option<&str>,
    dispatch_role: &str,
    action_name: &str,
    reason: &str,
    declared_files: &[String],
    child_execution_mode: &str,
    child_execution_label: Option<&str>,
    child_execution_text: Option<&str>,
) -> String {
    let mut lines = vec![
        format!("Own top-level task {task_id}: {task_title}"),
        format!("Role: {dispatch_role}"),
        format!("Category: {}", category_id.unwrap_or("unknown")),
        format!("Parent action: {action_name}"),
    ];

    match child_execution_mode {
        "current-step" => {
            if let Some(step_label) = child_execution_label {
                match child_execution_text {
                    Some(step_text) if !step_text.is_empty() => {
                        lines.push(format!("Current step: {step_label} - {step_text}"));
                    }
                    _ => lines.push(format!("Current step: {step_label}")),
                }
            }
            lines.push(
                "Execution boundary: own only this current step on this resume and return after that bounded step or a blocker."
                    .to_string(),
            );
        }
        "review-pass" => lines.push(
            "Execution boundary: run one review pass for this task and return explicit findings or a pass result."
                .to_string(),
        ),
        _ => lines.push(
            "Execution boundary: follow the returned task boundary exactly and return once that bounded work is finished or blocked."
                .to_string(),
        ),
    }

    if !declared_files.is_empty() {
        lines.push(format!("Files: {}", declared_files.join(", ")));
    }

    lines.push(format!("Why now: {reason}"));
    lines.push(
        "Do not mark the top-level task complete; return control to the parent with verification evidence and remaining risks."
            .to_string(),
    );
    lines.join("\n")
}

fn with_delegation_metadata(
    category: Option<&CategoryDefinition>,
    action: RawActionState,
) -> ActionState {
    let parent_only_actions: HashSet<&str> = [
        "acquire_write_lease",
        "accept_task",
        "complete_final_acceptance",
        "complete_plan",
    ]
    .into_iter()
    .collect();
    let delegation_preference = category
        .map(|entry| entry.delegation_preference.clone())
        .unwrap_or(DelegationPreference::ParentOnly);
    let requires_subagent = delegation_preference != DelegationPreference::ParentOnly
        && action.required_role.is_some()
        && !parent_only_actions.contains(action.action.as_str());

    if parent_only_actions.contains(action.action.as_str())
        || action.required_role.is_none()
        || delegation_preference == DelegationPreference::ParentOnly
    {
        let blocking_actions = blocking_control_plane_actions(category, &action);
        let (child_execution_mode, child_execution_label, child_execution_text) =
            child_execution_scope(&action.action, &action.step_progress);
        return ActionState {
            task_id: action.task_id,
            open_acceptance_items: action.open_acceptance_items,
            action: action.action.clone(),
            category_id: action.category_id,
            required_role: action.required_role,
            requires_write_lease: action.requires_write_lease,
            reason: action.reason,
            requires_subagent: false,
            dispatch_role: None,
            intervention_reason: if parent_only_actions.contains(action.action.as_str()) {
                format!("Action {} is a parent-owned control-plane step.", action.action)
            } else if delegation_preference == DelegationPreference::ParentOnly {
                format!(
                    "Category {} is configured for parent-local execution.",
                    category.map(|entry| entry.id.as_str()).unwrap_or("unknown")
                )
            } else {
                "No child role is required for this action.".to_string()
            },
            dispatch_mode: "parent-local".to_string(),
            task_session_mode: "parent-local".to_string(),
            task_session_key: None,
            continue_agent_id: None,
            subagent_tool_action: "none".to_string(),
            subagent_agent_type: None,
            subagent_dispatch_message: None,
            blocking_control_plane_actions: blocking_actions,
            child_execution_mode,
            child_execution_label,
            child_execution_text,
            step_progress: action.step_progress,
        };
    }

    let blocking_actions = blocking_control_plane_actions(category, &action);
    let (child_execution_mode, child_execution_label, child_execution_text) =
        child_execution_scope(&action.action, &action.step_progress);
    let dispatch_role = action.required_role.clone();
    ActionState {
        task_id: action.task_id,
        open_acceptance_items: action.open_acceptance_items,
        action: action.action,
        category_id: action.category_id,
        required_role: action.required_role.clone(),
        requires_write_lease: action.requires_write_lease,
        reason: action.reason,
        requires_subagent: true,
        dispatch_role,
        intervention_reason: if delegation_preference == DelegationPreference::SubagentRequired {
            format!(
                "Category {} requires subagent execution for normal task work.",
                category.map(|entry| entry.id.as_str()).unwrap_or("unknown")
            )
        } else {
            format!(
                "Category {} prefers subagent execution by default.",
                category.map(|entry| entry.id.as_str()).unwrap_or("unknown")
            )
        },
        dispatch_mode: derive_dispatch_mode(category, requires_subagent),
        task_session_mode: "spawn-dedicated-task-subagent".to_string(),
        task_session_key: None,
        continue_agent_id: None,
        subagent_tool_action: "spawn_agent".to_string(),
        subagent_agent_type: action.required_role.clone(),
        subagent_dispatch_message: None,
        blocking_control_plane_actions: blocking_actions,
        child_execution_mode,
        child_execution_label,
        child_execution_text,
        step_progress: action.step_progress,
    }
}

fn derive_dispatch_mode(category: Option<&CategoryDefinition>, requires_subagent: bool) -> String {
    if !requires_subagent {
        return "parent-local".to_string();
    }
    match category.map(|entry| entry.parallelism.as_str()) {
        Some("parallel") => "parallel-subagents".to_string(),
        Some("write-scope") => "write-scope-subagent".to_string(),
        _ => "single-subagent".to_string(),
    }
}

fn decorate_action_with_task_session(
    plan_id: &str,
    plan_task: &PlanTask,
    task_state: Option<&TaskStateRecord>,
    mut action: ActionState,
) -> ActionState {
    let Some(lane) = task_session_lane(&action, task_state) else {
        action.task_session_mode = "parent-local".to_string();
        action.task_session_key = None;
        action.continue_agent_id = None;
        action.subagent_tool_action = "none".to_string();
        action.subagent_agent_type = None;
        action.subagent_dispatch_message = None;
        return action;
    };

    let owner_agent_id = match lane {
        TaskSessionLane::Implementer => task_state.and_then(|entry| entry.implementation_agent_id.clone()),
        TaskSessionLane::Reviewer => task_state.and_then(|entry| entry.review_agent_id.clone()),
    };
    let mode = match lane {
        TaskSessionLane::Implementer => {
            if owner_agent_id.is_some() {
                "resume-dedicated-task-subagent"
            } else {
                "spawn-dedicated-task-subagent"
            }
        }
        TaskSessionLane::Reviewer => {
            if action.action == "continue_same_agent" && owner_agent_id.is_some() {
                "resume-dedicated-reviewer-subagent"
            } else {
                "spawn-dedicated-reviewer-subagent"
            }
        }
    };

    action.task_session_mode = mode.to_string();
    action.task_session_key = Some(task_session_key(plan_id, &plan_task.id, lane));
    action.continue_agent_id = if mode.starts_with("resume-") {
        owner_agent_id
    } else {
        None
    };
    action.subagent_tool_action = subagent_tool_action(mode);
    action.subagent_agent_type = action.dispatch_role.clone().or(action.required_role.clone());
    action.subagent_dispatch_message = action
        .subagent_agent_type
        .as_deref()
        .map(|dispatch_role| {
            subagent_dispatch_message(
                &plan_task.id,
                &plan_task.title,
                action.category_id.as_deref().or(Some(plan_task.category.as_str())),
                dispatch_role,
                &action.action,
                &action.reason,
                &plan_task.declared_files,
                &action.child_execution_mode,
                action.child_execution_label.as_deref(),
                action.child_execution_text.as_deref(),
            )
        });
    action
}

fn task_session_lane(
    action: &ActionState,
    task_state: Option<&TaskStateRecord>,
) -> Option<TaskSessionLane> {
    if !action.requires_subagent {
        return None;
    }
    if matches!(action.action.as_str(), "run_spec_review" | "run_quality_review") {
        return Some(TaskSessionLane::Reviewer);
    }
    if action.action == "continue_same_agent" {
        if matches!(
            task_state.map(|entry| entry.status.as_str()),
            Some("running_spec_review") | Some("running_quality_review")
        ) {
            return Some(TaskSessionLane::Reviewer);
        }
        return Some(TaskSessionLane::Implementer);
    }
    if matches!(
        action.action.as_str(),
        "dispatch_task" | "return_to_implementer" | "repair_and_re_review"
    ) {
        return Some(TaskSessionLane::Implementer);
    }
    None
}

fn task_session_key(plan_id: &str, task_id: &str, lane: TaskSessionLane) -> String {
    let lane_name = match lane {
        TaskSessionLane::Implementer => "implementer",
        TaskSessionLane::Reviewer => "review",
    };
    format!("task::{plan_id}::{task_id}::{lane_name}")
}

fn dispatch_entry_session(
    plan_id: &str,
    plan_task: &PlanTask,
    task_state: Option<&TaskStateRecord>,
    child_action: &str,
) -> (String, String, Option<String>) {
    let lane = if matches!(child_action, "run_spec_review" | "run_quality_review") {
        TaskSessionLane::Reviewer
    } else {
        TaskSessionLane::Implementer
    };
    let owner_agent_id = match lane {
        TaskSessionLane::Implementer => task_state.and_then(|entry| entry.implementation_agent_id.clone()),
        TaskSessionLane::Reviewer => task_state.and_then(|entry| entry.review_agent_id.clone()),
    };
    let mode = match lane {
        TaskSessionLane::Implementer => {
            if owner_agent_id.is_some() {
                "resume-dedicated-task-subagent"
            } else {
                "spawn-dedicated-task-subagent"
            }
        }
        TaskSessionLane::Reviewer => "spawn-dedicated-reviewer-subagent",
    };
    (
        mode.to_string(),
        task_session_key(plan_id, &plan_task.id, lane),
        if mode.starts_with("resume-") {
            owner_agent_id
        } else {
            None
        },
    )
}

fn shape_next_action_payload(
    plan_id: &str,
    action: &ActionState,
    task_id: Option<&str>,
    parallel_dispatches: &[ParallelDispatchEntry],
) -> Value {
    json!({
        "plan_id": plan_id,
        "task_id": task_id.map(|value| value.to_string()).or_else(|| action.task_id.clone()),
        "action": action.action,
        "category_id": action.category_id,
        "required_role": action.required_role,
        "requires_write_lease": action.requires_write_lease,
        "reason": action.reason,
        "requires_subagent": action.requires_subagent,
        "dispatch_role": action.dispatch_role,
        "intervention_reason": action.intervention_reason,
        "dispatch_mode": action.dispatch_mode,
        "task_session_mode": action.task_session_mode,
        "task_session_key": action.task_session_key,
        "continue_agent_id": action.continue_agent_id,
        "subagent_tool_action": action.subagent_tool_action,
        "subagent_agent_type": action.subagent_agent_type,
        "subagent_dispatch_message": action.subagent_dispatch_message,
        "blocking_control_plane_actions": action.blocking_control_plane_actions,
        "child_execution_mode": action.child_execution_mode,
        "child_execution_label": action.child_execution_label,
        "child_execution_text": action.child_execution_text,
        "open_acceptance_items": action.open_acceptance_items,
        "current_step_label": action.step_progress.current_step_label,
        "current_step_text": action.step_progress.current_step_text,
        "next_step_label": action.step_progress.next_step_label,
        "next_step_text": action.step_progress.next_step_text,
        "remaining_step_count": action.step_progress.remaining_step_count,
        "step_sync_status": action.step_progress.step_sync_status,
        "step_sync_action": action.step_progress.step_sync_action,
        "parallel_task_ids": parallel_dispatches.iter().map(|entry| entry.task_id.clone()).collect::<Vec<_>>(),
        "parallel_dispatches": parallel_dispatches,
    })
}

fn plan_state_payload(plan_state: &PlanState) -> Value {
    json!({
        "executionStatus": {
            "currentWave": plan_state.execution_status.current_wave,
            "activeTask": plan_state.execution_status.active_task,
            "blockers": plan_state.execution_status.blockers,
            "lastReviewResult": plan_state.execution_status.last_review_result,
        },
        "tasks": plan_state.tasks.iter().map(|task| json!({
            "id": task.id,
            "title": task.title,
            "dependsOn": task.depends_on,
            "declaredFiles": task.declared_files,
            "category": task.category,
            "ownerRole": task.owner_role,
            "taskStatus": task.task_status,
            "currentStep": task.current_step,
            "specReviewStatus": task.spec_review_status,
            "qualityReviewStatus": task.quality_review_status,
            "assignedAgent": task.assigned_agent,
            "todoChecked": task.todo_checked,
            "steps": task.steps.iter().map(|step| json!({
                "label": step.label,
                "text": step.text,
                "checked": step.checked,
            })).collect::<Vec<_>>(),
        })).collect::<Vec<_>>(),
    })
}

fn first_dependency_ready_open_task(plan_state: &PlanState) -> Option<PlanTask> {
    let completed = completed_task_ids(plan_state);
    plan_state
        .tasks
        .iter()
        .filter(|task| !task.todo_checked && dependencies_satisfied(task, &completed))
        .cloned()
        .next()
}

fn completed_task_ids(plan_state: &PlanState) -> HashSet<String> {
    plan_state
        .tasks
        .iter()
        .filter(|task| task.todo_checked)
        .map(|task| task.id.clone())
        .collect()
}

fn dependencies_satisfied(task: &PlanTask, completed: &HashSet<String>) -> bool {
    task.depends_on.iter().all(|task_id| completed.contains(task_id))
}

fn build_parallel_dispatch_batch(
    plan_state: &PlanState,
    runtime_store: &RuntimeStore,
    categories: &CategoryRegistry,
    plan_id: &str,
    primary_task: &PlanTask,
    primary_action: &ActionState,
    final_acceptance_open: &[String],
) -> Result<Option<ParallelBatchPlan>> {
    let primary_category = categories.get(&primary_task.category);
    let Some(primary_mode) = parallel_batch_mode(primary_task, primary_action, primary_category) else {
        return Ok(None);
    };
    if primary_mode.dispatch_mode == "write-scope-subagent"
        && child_dispatch_scope(primary_task).is_empty()
    {
        return Ok(None);
    }

    let completed = completed_task_ids(plan_state);
    let mut selected = Vec::new();
    let mut occupied_scopes: Vec<Vec<String>> = Vec::new();

    for task in plan_state.tasks.iter().filter(|task| !task.todo_checked) {
        if !dependencies_satisfied(task, &completed) {
            continue;
        }

        let task_state = runtime_store.get_task_state(plan_id, &task.id)?;
        let active_lease = runtime_store.get_active_write_lease(plan_id, &task.id)?;
        let category = categories.get(&task.category);
        let action = derive_next_action(
            task,
            task_state.as_ref(),
            category,
            active_lease.is_some(),
            final_acceptance_open,
        );
        let action = decorate_action_with_task_session(plan_id, task, task_state.as_ref(), action);
        let Some(candidate_mode) = parallel_batch_mode(task, &action, category) else {
            continue;
        };
        if !is_parallel_batch_compatible(&primary_mode, &candidate_mode) {
            continue;
        }

        let dispatch_scope = child_dispatch_scope(task);
        if candidate_mode.dispatch_mode == "write-scope-subagent" && dispatch_scope.is_empty() {
            continue;
        }
        if occupied_scopes
            .iter()
            .any(|existing_scope| scopes_conflict(existing_scope, &dispatch_scope))
        {
            continue;
        }

        if !dispatch_scope.is_empty() {
            occupied_scopes.push(dispatch_scope.clone());
        }
        let (task_session_mode, task_session_key, continue_agent_id) =
            dispatch_entry_session(plan_id, task, task_state.as_ref(), &candidate_mode.child_action);
        let synthetic_step_progress = build_step_progress(task, task_state.clone());
        let synthetic_child_action = RawActionState {
            task_id: Some(task.id.clone()),
            open_acceptance_items: Vec::new(),
            action: candidate_mode.child_action.clone(),
            category_id: Some(task.category.clone()),
            required_role: Some(candidate_mode.dispatch_role.clone()),
            requires_write_lease: candidate_mode.requires_write_lease,
            reason: action.reason.clone(),
            step_progress: synthetic_step_progress.clone(),
        };
        let blocking_control_plane_actions =
            blocking_control_plane_actions(category, &synthetic_child_action);
        let (child_execution_mode, child_execution_label, child_execution_text) =
            child_execution_scope(&synthetic_child_action.action, &synthetic_child_action.step_progress);
        let subagent_tool_action = subagent_tool_action(&task_session_mode);
        let subagent_dispatch_message = subagent_dispatch_message(
            &task.id,
            &task.title,
            Some(task.category.as_str()),
            &candidate_mode.dispatch_role,
            &candidate_mode.child_action,
            &action.reason,
            &task.declared_files,
            &child_execution_mode,
            child_execution_label.as_deref(),
            child_execution_text.as_deref(),
        );
        selected.push(ParallelDispatchEntry {
            task_id: task.id.clone(),
            title: task.title.clone(),
            action: candidate_mode.child_action.clone(),
            category_id: task.category.clone(),
            dispatch_role: candidate_mode.dispatch_role.clone(),
            dispatch_mode: candidate_mode.dispatch_mode.clone(),
            task_session_mode,
            task_session_key,
            continue_agent_id,
            subagent_tool_action,
            subagent_agent_type: candidate_mode.dispatch_role.clone(),
            subagent_dispatch_message,
            blocking_control_plane_actions,
            child_execution_mode,
            child_execution_label,
            child_execution_text,
            requires_write_lease: candidate_mode.requires_write_lease,
            dispatch_scope,
            depends_on: task.depends_on.clone(),
            reason: action.reason.clone(),
        });
    }

    if selected.first().map(|entry| entry.task_id.as_str()) != Some(primary_task.id.as_str()) {
        return Ok(None);
    }

    Ok(Some(ParallelBatchPlan {
        top_level_action: primary_mode.top_level_action,
        entries: selected,
    }))
}

#[derive(Debug, Clone)]
struct ParallelBatchMode {
    top_level_action: String,
    child_action: String,
    dispatch_role: String,
    dispatch_mode: String,
    requires_write_lease: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TaskSessionLane {
    Implementer,
    Reviewer,
}

fn parallel_batch_mode(
    task: &PlanTask,
    action: &ActionState,
    category: Option<&CategoryDefinition>,
) -> Option<ParallelBatchMode> {
    if action.requires_subagent
        && matches!(
            action.action.as_str(),
            "dispatch_task" | "run_spec_review" | "run_quality_review" | "repair_and_re_review" | "return_to_implementer"
        )
    {
        return Some(ParallelBatchMode {
            top_level_action: "dispatch_parallel_tasks".to_string(),
            child_action: action.action.clone(),
            dispatch_role: action
                .dispatch_role
                .clone()
                .or(action.required_role.clone())
                .unwrap_or_else(|| task.owner_role.clone()),
            dispatch_mode: action.dispatch_mode.clone(),
            requires_write_lease: action.requires_write_lease,
        });
    }

    if action.action == "acquire_write_lease"
        && category.map(|entry| entry.parallelism.as_str()) == Some("write-scope")
        && category
            .map(|entry| entry.delegation_preference != DelegationPreference::ParentOnly)
            .unwrap_or(false)
    {
        return Some(ParallelBatchMode {
            top_level_action: "acquire_parallel_write_leases".to_string(),
            child_action: "dispatch_task".to_string(),
            dispatch_role: task.owner_role.clone(),
            dispatch_mode: "write-scope-subagent".to_string(),
            requires_write_lease: true,
        });
    }

    None
}

fn is_parallel_batch_compatible(primary: &ParallelBatchMode, candidate: &ParallelBatchMode) -> bool {
    candidate.top_level_action == primary.top_level_action
        && candidate.child_action == primary.child_action
        && candidate.dispatch_mode == primary.dispatch_mode
}

fn child_dispatch_scope(task: &PlanTask) -> Vec<String> {
    let mut scope = task
        .declared_files
        .iter()
        .map(|path| normalize_repo_path(path))
        .filter(|path| !is_parent_owned_coordination_path(path))
        .collect::<Vec<_>>();
    scope.sort();
    scope.dedup();
    scope
}

fn normalize_repo_path(path: &str) -> String {
    path.replace('\\', "/")
        .trim()
        .trim_start_matches("./")
        .trim_matches('`')
        .to_lowercase()
}

fn is_parent_owned_coordination_path(path: &str) -> bool {
    matches!(
        path,
        "task_plan.md" | "progress.md" | "findings.md" | "agents.md" | "docs/index.md"
    ) || path.starts_with("docs/plans/active/")
        || path.starts_with("docs/plans/completed/")
}

fn scopes_conflict(left: &[String], right: &[String]) -> bool {
    if left.is_empty() || right.is_empty() {
        return false;
    }
    left.iter()
        .any(|left_path| right.iter().any(|right_path| repo_paths_conflict(left_path, right_path)))
}

fn repo_paths_conflict(left: &str, right: &str) -> bool {
    if left == right {
        return true;
    }

    let left_prefix = wildcard_prefix(left);
    let right_prefix = wildcard_prefix(right);

    if let Some(prefix) = left_prefix.as_deref() {
        if path_within_prefix(right, prefix) {
            return true;
        }
    }
    if let Some(prefix) = right_prefix.as_deref() {
        if path_within_prefix(left, prefix) {
            return true;
        }
    }

    false
}

fn wildcard_prefix(path: &str) -> Option<String> {
    path.find('*')
        .map(|index| path[..index].trim_end_matches('/').to_string())
        .filter(|prefix| !prefix.is_empty())
}

fn path_within_prefix(path: &str, prefix: &str) -> bool {
    path == prefix || path.starts_with(&format!("{prefix}/"))
}

fn require_plan_task(plan_state: &PlanState, task_id: &str) -> Result<PlanTask> {
    plan_state
        .tasks
        .iter()
        .find(|entry| entry.id == task_id)
        .cloned()
        .ok_or_else(|| anyhow!("Task not found in plan: {task_id}"))
}
