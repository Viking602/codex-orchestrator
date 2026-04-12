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
    required_role: Option<String>,
    requires_write_lease: bool,
    reason: String,
    requires_subagent: bool,
    dispatch_role: Option<String>,
    intervention_reason: String,
    dispatch_mode: String,
    step_progress: StepProgressState,
}

#[derive(Debug, Clone)]
struct RawActionState {
    task_id: Option<String>,
    open_acceptance_items: Vec<String>,
    action: String,
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
            description: "Record a spec or quality review result and update task metadata.",
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
            if reviewer_agent_id.as_deref() == current.agent_id.as_deref()
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
            tool_result(json!({
                "plan_id": plan_id,
                "task_id": task_id,
                "review_type": review_type,
                "result": review_result,
                "task_status": next_task_status,
            }))
        }
        "orchestrator_accept_task" => {
            let mut plan = PlanDocument::new(&require_string(args, "planPath")?);
            let task_id = require_string(args, "taskId")?;
            let plan_id = plan_id_from_path(plan.plan_path())?;
            sync_stored_plan_path(&ctx.runtime_store, &plan_id, plan.plan_path())?;
            let current = ctx
                .runtime_store
                .get_task_state(&plan_id, &task_id)?
                .ok_or_else(|| anyhow!("Task state missing: {task_id}"))?;
            if !plan.all_steps_completed(&task_id)? {
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
            ctx.runtime_store.upsert_task_state(next)?;
            plan.update_task_metadata(&task_id, Some("accepted"), Some("none"), None, None, None)?;
            plan.mark_top_level_todo(&task_id, true)?;
            tool_result(json!({
                "plan_id": plan_id,
                "task_id": task_id,
                "accepted": true,
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
            let next_task = plan_state.tasks.iter().find(|task| !task.todo_checked).cloned();
            if let Some(next_task) = next_task {
                let task_state = ctx.runtime_store.get_task_state(&plan_id, &next_task.id)?;
                let active_lease = ctx.runtime_store.get_active_write_lease(&plan_id, &next_task.id)?;
                let action = derive_next_action(
                    &next_task,
                    task_state.as_ref(),
                    ctx.categories.get(&next_task.category),
                    active_lease.is_some(),
                    &plan.unchecked_final_acceptance_items()?,
                );
                return tool_result(shape_next_action_payload(&plan_id, &action, Some(&next_task.id)));
            }
            let open_acceptance = plan.unchecked_final_acceptance_items()?;
            if !open_acceptance.is_empty() {
                let action = with_delegation_metadata(
                    None,
                    RawActionState {
                        task_id: None,
                        open_acceptance_items: open_acceptance.clone(),
                        action: "complete_final_acceptance".to_string(),
                        required_role: None,
                        requires_write_lease: false,
                        reason: format!(
                            "All top-level tasks are accepted, but final acceptance still has open items: {}.",
                            open_acceptance.join(", ")
                        ),
                        step_progress: empty_step_progress(),
                    },
                );
                return tool_result(shape_next_action_payload(&plan_id, &action, None));
            }
            let action = with_delegation_metadata(
                None,
                RawActionState {
                    task_id: None,
                    open_acceptance_items: Vec::new(),
                    action: "complete_plan".to_string(),
                    required_role: None,
                    requires_write_lease: false,
                    reason: "All top-level tasks are already accepted.".to_string(),
                    step_progress: empty_step_progress(),
                },
            );
            tool_result(shape_next_action_payload(&plan_id, &action, None))
        }
        "orchestrator_question_gate" => {
            let question_category = require_string(args, "questionCategory")?;
            let user_explicitly_requested = optional_bool(args, "userExplicitlyRequested").unwrap_or(false);
            let reason = optional_string(args, "reason").unwrap_or_default();
            let hard_blockers: HashSet<&str> =
                ["identity", "credential", "destructive", "conflict"]
                    .into_iter()
                    .collect();
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

fn task_upsert_from_current(current: &TaskStateRecord) -> TaskStateUpsertInput {
    TaskStateUpsertInput {
        plan_id: current.plan_id.clone(),
        task_id: current.task_id.clone(),
        category_id: current.category_id.clone(),
        status: current.status.clone(),
        active_step_label: current.active_step_label.clone(),
        assigned_role: current.assigned_role.clone(),
        agent_id: current.agent_id.clone(),
        write_lease_id: current.write_lease_id.clone(),
        spec_review_status: current.spec_review_status.clone(),
        quality_review_status: current.quality_review_status.clone(),
        retry_count: current.retry_count,
        blocker_type: current.blocker_type.clone(),
        blocker_message: current.blocker_message.clone(),
    }
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
                task_id: None,
                open_acceptance_items: Vec::new(),
                action: "acquire_write_lease".to_string(),
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
                task_id: None,
                open_acceptance_items: Vec::new(),
                action: if plan_task.spec_review_status == "fail" {
                    "repair_and_re_review".to_string()
                } else {
                    "run_spec_review".to_string()
                },
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
                task_id: None,
                open_acceptance_items: Vec::new(),
                action: if plan_task.quality_review_status == "fail" {
                    "repair_and_re_review".to_string()
                } else {
                    "run_quality_review".to_string()
                },
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
                task_id: None,
                open_acceptance_items: Vec::new(),
                action: "return_to_implementer".to_string(),
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
                task_id: None,
                open_acceptance_items: Vec::new(),
                action: "accept_task".to_string(),
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
                task_id: None,
                open_acceptance_items: Vec::new(),
                action: "run_spec_review".to_string(),
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
                task_id: None,
                open_acceptance_items: Vec::new(),
                action: "run_quality_review".to_string(),
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
                task_id: None,
                open_acceptance_items: final_acceptance_open.to_vec(),
                action: "complete_final_acceptance".to_string(),
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
                task_id: None,
                open_acceptance_items: Vec::new(),
                action: "continue_same_agent".to_string(),
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
                task_id: None,
                open_acceptance_items: Vec::new(),
                action: "continue_same_agent".to_string(),
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
            task_id: None,
            open_acceptance_items: Vec::new(),
            action: "dispatch_task".to_string(),
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
        return ActionState {
            task_id: action.task_id,
            open_acceptance_items: action.open_acceptance_items,
            action: action.action.clone(),
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
            step_progress: action.step_progress,
        };
    }

    ActionState {
        task_id: action.task_id,
        open_acceptance_items: action.open_acceptance_items,
        action: action.action,
        required_role: action.required_role.clone(),
        requires_write_lease: action.requires_write_lease,
        reason: action.reason,
        requires_subagent: true,
        dispatch_role: action.required_role,
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

fn shape_next_action_payload(plan_id: &str, action: &ActionState, task_id: Option<&str>) -> Value {
    json!({
        "plan_id": plan_id,
        "task_id": task_id.map(|value| value.to_string()).or_else(|| action.task_id.clone()),
        "action": action.action,
        "required_role": action.required_role,
        "requires_write_lease": action.requires_write_lease,
        "reason": action.reason,
        "requires_subagent": action.requires_subagent,
        "dispatch_role": action.dispatch_role,
        "intervention_reason": action.intervention_reason,
        "dispatch_mode": action.dispatch_mode,
        "open_acceptance_items": action.open_acceptance_items,
        "current_step_label": action.step_progress.current_step_label,
        "current_step_text": action.step_progress.current_step_text,
        "next_step_label": action.step_progress.next_step_label,
        "next_step_text": action.step_progress.next_step_text,
        "remaining_step_count": action.step_progress.remaining_step_count,
        "step_sync_status": action.step_progress.step_sync_status,
        "step_sync_action": action.step_progress.step_sync_action,
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

fn require_plan_task(plan_state: &PlanState, task_id: &str) -> Result<PlanTask> {
    plan_state
        .tasks
        .iter()
        .find(|entry| entry.id == task_id)
        .cloned()
        .ok_or_else(|| anyhow!("Task not found in plan: {task_id}"))
}
