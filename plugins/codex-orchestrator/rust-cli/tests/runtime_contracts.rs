use std::fs;

use codex_orchestrator_mcp::{
    category_registry::CategoryRegistry,
    runtime_store::{RuntimeStore, TaskStateUpsertInput},
    tools::{handle_tool_call, AppContext},
};
use serde_json::{json, Map, Value};
use tempfile::tempdir;

fn write_categories(path: &std::path::Path) {
    fs::write(
        path,
        r#"
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
"#,
    )
    .unwrap();
}

fn write_file(path: &std::path::Path, contents: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(path, contents).unwrap();
}

fn tool_args(value: Value) -> Map<String, Value> {
    value.as_object().cloned().unwrap()
}

fn structured(result: Value) -> Value {
    result.get("structuredContent").cloned().unwrap()
}

fn create_context(base: &std::path::Path) -> AppContext {
    let categories = base.join("categories.toml");
    let db = base.join("state.db");
    write_categories(&categories);
    AppContext {
        categories: CategoryRegistry::from_toml(categories.to_str().unwrap()).unwrap(),
        runtime_store: RuntimeStore::new(db.to_str().unwrap()).unwrap(),
    }
}

fn codex_todo_mirror_plan() -> &'static str {
    r#"# Codex Todo Mirror Plan

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
"#
}

fn final_acceptance_plan() -> &'static str {
    r#"# Final Acceptance Plan

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
"#
}

fn phase3_plan() -> &'static str {
    r#"# Phase 3 Plan

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
"#
}

#[test]
fn resolve_category_exposes_default_subagent_bias() {
    let temp = tempdir().unwrap();
    let ctx = create_context(temp.path());

    let payload = structured(
        handle_tool_call(
            &ctx,
            "orchestrator_resolve_category",
            &tool_args(json!({
                "title": "Review current runtime",
                "description": "Review and verify orchestration behavior"
            })),
        )
        .unwrap(),
    );

    assert_eq!(payload["category_id"], "review");
    assert_eq!(payload["preferred_role"], "harness-evaluator");
    assert_eq!(payload["delegation_preference"], "subagent-required");
    assert_eq!(payload["requires_subagent_default"], true);
}

#[test]
fn export_codex_todo_mirrors_completed_active_and_pending_work() {
    let temp = tempdir().unwrap();
    let plan = temp.path().join("codex-todo-plan.md");
    write_file(&plan, codex_todo_mirror_plan());
    let ctx = create_context(temp.path());
    ctx.runtime_store
        .upsert_task_state(TaskStateUpsertInput {
            plan_id: "codex-todo-plan".to_string(),
            task_id: "A1".to_string(),
            category_id: "backend-impl".to_string(),
            status: "running_impl".to_string(),
            active_step_label: Some("Step 1".to_string()),
            assigned_role: Some("backend-developer".to_string()),
            agent_id: Some("agent-impl".to_string()),
            write_lease_id: None,
            spec_review_status: "pending".to_string(),
            quality_review_status: "pending".to_string(),
            retry_count: 0,
            blocker_type: None,
            blocker_message: None,
        })
        .unwrap();

    let payload = structured(
        handle_tool_call(
            &ctx,
            "orchestrator_export_codex_todo",
            &tool_args(json!({ "planPath": plan })),
        )
        .unwrap(),
    );

    let items = payload["items"].as_array().unwrap();
    assert_eq!(payload["active_task_id"], "A1");
    assert_eq!(payload["current_step_label"], "Step 1");
    assert_eq!(payload["current_step_text"], "Implement export tool");
    assert_eq!(payload["step_sync_status"], "step_in_progress");
    assert_eq!(items[0]["status"], "completed");
    assert_eq!(items[1]["status"], "in_progress");
    assert_eq!(items[2]["status"], "pending");
    assert!(items[1]["step"]
        .as_str()
        .unwrap()
        .contains("A1. Active Implementation Task"));
}

#[test]
fn export_codex_todo_mirrors_final_acceptance_when_only_acceptance_remains() {
    let temp = tempdir().unwrap();
    let plan = temp.path().join("final-acceptance-plan.md");
    write_file(&plan, final_acceptance_plan());
    let ctx = create_context(temp.path());

    let payload = structured(
        handle_tool_call(
            &ctx,
            "orchestrator_export_codex_todo",
            &tool_args(json!({ "planPath": plan })),
        )
        .unwrap(),
    );

    let items = payload["items"].as_array().unwrap();
    let open_acceptance = payload["open_acceptance_items"].as_array().unwrap();
    assert_eq!(items.len(), 2);
    assert_eq!(items[0]["status"], "completed");
    assert_eq!(items[1]["status"], "in_progress");
    assert!(items[1]["step"]
        .as_str()
        .unwrap()
        .to_lowercase()
        .contains("final acceptance"));
    assert_eq!(open_acceptance[0], "final acceptance");
}

#[test]
fn question_gate_rejects_optional_expansion_by_default() {
    let temp = tempdir().unwrap();
    let ctx = create_context(temp.path());

    let payload = structured(
        handle_tool_call(
            &ctx,
            "orchestrator_question_gate",
            &tool_args(json!({
                "questionCategory": "optional_expansion",
                "userExplicitlyRequested": false,
                "reason": "Could ask whether to add extra analytics support"
            })),
        )
        .unwrap(),
    );

    assert_eq!(payload["ask_user"], false);
    assert_eq!(payload["allowed_to_expand"], false);
}

#[test]
fn completion_assessment_blocks_acceptance_when_steps_and_evidence_are_missing() {
    let temp = tempdir().unwrap();
    let plan = temp.path().join("phase3-plan.md");
    write_file(&plan, phase3_plan());
    let ctx = create_context(temp.path());
    ctx.runtime_store
        .upsert_task_state(TaskStateUpsertInput {
            plan_id: "phase3-plan".to_string(),
            task_id: "P3-T3".to_string(),
            category_id: "backend-impl".to_string(),
            status: "impl_done".to_string(),
            active_step_label: None,
            assigned_role: Some("backend-developer".to_string()),
            agent_id: Some("agent-impl".to_string()),
            write_lease_id: None,
            spec_review_status: "pending".to_string(),
            quality_review_status: "pending".to_string(),
            retry_count: 0,
            blocker_type: None,
            blocker_message: None,
        })
        .unwrap();

    let payload = structured(
        handle_tool_call(
            &ctx,
            "orchestrator_assess_subagent_completion",
            &tool_args(json!({
                "planPath": plan,
                "taskId": "P3-T3"
            })),
        )
        .unwrap(),
    );

    let missing_steps = payload["missing_steps"].as_array().unwrap();
    assert_eq!(payload["implementation_complete"], false);
    assert_eq!(payload["missing_evidence"], true);
    assert_eq!(payload["can_accept"], false);
    assert_eq!(payload["next_required_stage"], "implementation");
    assert_eq!(missing_steps[0], "Step 1");
}

#[test]
fn completion_guard_fails_closed_when_final_acceptance_is_not_complete() {
    let temp = tempdir().unwrap();
    let plan = temp.path().join("phase3-plan.md");
    write_file(&plan, phase3_plan());
    let ctx = create_context(temp.path());

    let payload = structured(
        handle_tool_call(
            &ctx,
            "orchestrator_completion_guard",
            &tool_args(json!({ "planPath": plan })),
        )
        .unwrap(),
    );

    let open_tasks = payload["open_tasks"].as_array().unwrap();
    let open_acceptance = payload["open_acceptance_items"].as_array().unwrap();
    assert_eq!(payload["can_finish"], false);
    assert_eq!(open_tasks[0], "P3-T2");
    assert_eq!(open_acceptance[0], "all done");
}
