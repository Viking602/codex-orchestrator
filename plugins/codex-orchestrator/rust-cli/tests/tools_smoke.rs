use std::fs;

use codex_orchestrator_mcp::{
    category_registry::CategoryRegistry,
    runtime_store::{PlanStateUpsertInput, RuntimeStore, TaskStateUpsertInput},
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

fn multi_step_plan() -> &'static str {
    r#"# Multi Step Plan

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
"#
}

fn completed_active_plan() -> &'static str {
    r#"# Completed Active Plan

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
"#
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

#[test]
fn begin_task_seeds_first_unchecked_step() {
    let temp = tempdir().unwrap();
    let plan = temp.path().join("multi-step-plan.md");
    write_file(&plan, multi_step_plan());
    let ctx = create_context(temp.path());
    ctx.runtime_store
        .upsert_plan_state(PlanStateUpsertInput {
            plan_id: "multi-step-plan".to_string(),
            plan_path: plan.to_str().unwrap().to_string(),
            spec_path: None,
            current_wave: Some("Wave 1".to_string()),
            active_task_id: Some("P2-T2".to_string()),
            last_review_result: None,
        })
        .unwrap();
    ctx.runtime_store
        .acquire_write_lease(
            "multi-step-plan",
            "P2-T2",
            "agent-1",
            &[String::from("src/**")],
        )
        .unwrap();

    let payload = structured(
        handle_tool_call(
            &ctx,
            "orchestrator_begin_task",
            &tool_args(json!({
                "planPath": plan,
                "taskId": "P2-T2",
                "categoryId": "backend-impl",
                "role": "backend-developer",
                "assignedAgent": "agent-1"
            })),
        )
        .unwrap(),
    );

    assert_eq!(payload["current_step_label"], "Step 1");
    assert_eq!(payload["next_step_label"], "Step 1");
    assert_eq!(payload["remaining_step_count"], 2);
    assert_eq!(payload["step_sync_status"], "step_in_progress");
}

#[test]
fn next_action_requests_write_lease_before_dispatch() {
    let temp = tempdir().unwrap();
    let plan = temp.path().join("multi-step-plan.md");
    write_file(&plan, multi_step_plan());
    let ctx = create_context(temp.path());

    let payload = structured(
        handle_tool_call(
            &ctx,
            "orchestrator_next_action",
            &tool_args(json!({ "planPath": plan })),
        )
        .unwrap(),
    );

    assert_eq!(payload["task_id"], "P2-T2");
    assert_eq!(payload["action"], "acquire_write_lease");
    assert_eq!(payload["requires_write_lease"], true);
    assert_eq!(payload["requires_subagent"], false);
    assert_eq!(payload["dispatch_mode"], "parent-local");
}

#[test]
fn watchdog_uses_negative_threshold_as_match_all_and_prefers_step_sync_repair() {
    let temp = tempdir().unwrap();
    let plan = temp.path().join("multi-step-plan.md");
    write_file(&plan, multi_step_plan());
    let ctx = create_context(temp.path());
    ctx.runtime_store
        .upsert_plan_state(PlanStateUpsertInput {
            plan_id: "multi-step-plan".to_string(),
            plan_path: plan.to_str().unwrap().to_string(),
            spec_path: None,
            current_wave: Some("Wave 1".to_string()),
            active_task_id: Some("P2-T2".to_string()),
            last_review_result: None,
        })
        .unwrap();
    ctx.runtime_store
        .acquire_write_lease(
            "multi-step-plan",
            "P2-T2",
            "agent-1",
            &[String::from("src/**")],
        )
        .unwrap();
    ctx.runtime_store
        .upsert_task_state(TaskStateUpsertInput {
            plan_id: "multi-step-plan".to_string(),
            task_id: "P2-T2".to_string(),
            category_id: "backend-impl".to_string(),
            status: "running_impl".to_string(),
            active_step_label: None,
            assigned_role: Some("backend-developer".to_string()),
            agent_id: Some("agent-1".to_string()),
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
            "orchestrator_watchdog_tick",
            &tool_args(json!({ "planId": "multi-step-plan", "olderThanMs": -1 })),
        )
        .unwrap(),
    );

    assert_eq!(payload["stalled_tasks"][0]["task_id"], "P2-T2");
    assert_eq!(payload["stalled_tasks"][0]["suggested_action"], "repair_step_sync");
}

#[test]
fn read_plan_state_archives_completed_active_plan() {
    let temp = tempdir().unwrap();
    let active = temp
        .path()
        .join("docs")
        .join("plans")
        .join("active")
        .join("archived-by-read.md");
    let completed = temp
        .path()
        .join("docs")
        .join("plans")
        .join("completed")
        .join("archived-by-read.md");
    write_file(&active, completed_active_plan());
    let ctx = create_context(temp.path());

    let payload = structured(
        handle_tool_call(
            &ctx,
            "orchestrator_read_plan_state",
            &tool_args(json!({ "planPath": active })),
        )
        .unwrap(),
    );

    assert_eq!(payload["tasks"][0]["id"], "D1");
    assert!(!active.exists());
    assert!(completed.exists());
}
