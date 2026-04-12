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
[research]
intent = "research"
preferred_role = "search-specialist"
allowed_roles = ["search-specialist"]
write_policy = "read-only"
requires_plan = false
requires_spec_review = false
requires_quality_review = false
parallelism = "parallel"
delegation_preference = "subagent-required"
reuse_policy = "no_reuse"
completion_contract = ["findings_recorded"]

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

fn research_plan() -> &'static str {
    r#"# Research Plan

## Execution Status

- Current wave: Wave Inspect
- Active task: R1
- Blockers: None
- Last review result: Not started

## TODO List

- [ ] R1. Inspect Repository Surface

### Task R1: Inspect Repository Surface

**Category:** research
**Owner Role:** search-specialist
**Task Status:** planned
**Current Step:** none
**Spec Review Status:** pending
**Quality Review Status:** pending
**Assigned Agent:** unassigned

- [ ] Step 1: Gather read-only evidence from the repository

## Final Acceptance

- [ ] all done
"#
}

fn review_lane_plan() -> &'static str {
    r#"# Review Lane Plan

## Execution Status

- Current wave: Wave Review
- Active task: R1
- Blockers: None
- Last review result: Not started

## TODO List

- [ ] R1. Review This Task

### Task R1: Review This Task

**Category:** backend-impl
**Owner Role:** backend-developer
**Task Status:** impl_done
**Current Step:** none
**Spec Review Status:** pending
**Quality Review Status:** pending
**Assigned Agent:** agent-impl

- [x] Step 1: Finish the implementation

## Final Acceptance

- [ ] all done
"#
}

fn repair_lane_plan() -> &'static str {
    r#"# Repair Lane Plan

## Execution Status

- Current wave: Wave Repair
- Active task: Q1
- Blockers: None
- Last review result: spec fail

## TODO List

- [ ] Q1. Repair The Task

### Task Q1: Repair The Task

**Category:** backend-impl
**Owner Role:** backend-developer
**Task Status:** spec_failed
**Current Step:** none
**Spec Review Status:** fail
**Quality Review Status:** pending
**Assigned Agent:** agent-review

- [x] Step 1: Finish the implementation

## Final Acceptance

- [ ] all done
"#
}

fn step_sync_repair_plan() -> &'static str {
    r#"# Step Sync Repair Plan

## Execution Status

- Current wave: Wave Step Sync
- Active task: S1
- Blockers: None
- Last review result: Not started

## TODO List

- [ ] S1. Continue Implementation

### Task S1: Continue Implementation

**Category:** backend-impl
**Owner Role:** backend-developer
**Task Status:** running_impl
**Current Step:** none
**Spec Review Status:** pending
**Quality Review Status:** pending
**Assigned Agent:** agent-impl

- [ ] Step 1: Finish the next bounded coding step

## Final Acceptance

- [ ] all done
"#
}

fn parallel_dispatch_plan() -> &'static str {
    r#"# Parallel Dispatch Plan

## Execution Status

- Current wave: Wave Parallel
- Active task: none
- Blockers: None
- Last review result: Not started

## Task Dependency Graph

| Task | Depends On | Reason |
|---|---|---|
| T1. Build API transport | None | Independent implementation task |
| T2. Build storage adapter | None | Independent implementation task |
| T3. Extend API transport tests | None | Conflicts with T1 write scope |
| T4. Add integration coverage | T1 | Must wait for transport work |

## TODO List

- [ ] T1. Build API transport
- [ ] T2. Build storage adapter
- [ ] T3. Extend API transport tests
- [ ] T4. Add integration coverage

### Task T1: Build API transport

**Files:**
- Modify: `src/api.rs`
- Modify: `task_plan.md`

**Category:** backend-impl
**Owner Role:** backend-developer
**Task Status:** ready
**Current Step:** none
**Spec Review Status:** pending
**Quality Review Status:** pending
**Assigned Agent:** unassigned

- [ ] Step 1: Implement the transport entrypoint

### Task T2: Build storage adapter

**Files:**
- Modify: `src/storage.rs`
- Modify: `progress.md`

**Category:** backend-impl
**Owner Role:** backend-developer
**Task Status:** ready
**Current Step:** none
**Spec Review Status:** pending
**Quality Review Status:** pending
**Assigned Agent:** unassigned

- [ ] Step 1: Implement the storage adapter

### Task T3: Extend API transport tests

**Files:**
- Modify: `src/api.rs`
- Modify: `tests/api.rs`

**Category:** backend-impl
**Owner Role:** backend-developer
**Task Status:** ready
**Current Step:** none
**Spec Review Status:** pending
**Quality Review Status:** pending
**Assigned Agent:** unassigned

- [ ] Step 1: Extend transport test coverage

### Task T4: Add integration coverage

**Files:**
- Modify: `tests/integration.rs`

**Category:** backend-impl
**Owner Role:** backend-developer
**Task Status:** ready
**Current Step:** none
**Spec Review Status:** pending
**Quality Review Status:** pending
**Assigned Agent:** unassigned

- [ ] Step 1: Add integration coverage after transport is ready

## Final Acceptance

- [ ] all done
"#
}

fn immediate_acceptance_plan() -> &'static str {
    r#"# Immediate Acceptance Plan

## Execution Status

- Current wave: Wave Accept
- Active task: A1
- Blockers: None
- Last review result: Not started

## TODO List

- [ ] A1. Finish Current Task
- [ ] B1. Start Follow-Up Task

### Task A1: Finish Current Task

**Category:** backend-impl
**Owner Role:** backend-developer
**Task Status:** running_quality_review
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pending
**Assigned Agent:** agent-impl

- [x] Step 1: Finish implementation

### Task B1: Start Follow-Up Task

**Category:** backend-impl
**Owner Role:** backend-developer
**Task Status:** ready
**Current Step:** none
**Spec Review Status:** pending
**Quality Review Status:** pending
**Assigned Agent:** unassigned

- [ ] Step 1: Start the next implementation task

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
fn resolve_category_routes_codebase_checks_to_research() {
    let temp = tempdir().unwrap();
    let ctx = create_context(temp.path());

    let english = structured(
        handle_tool_call(
            &ctx,
            "orchestrator_resolve_category",
            &tool_args(json!({
                "title": "Check this codebase",
                "description": "Audit the repository and summarize the key modules before we change anything"
            })),
        )
        .unwrap(),
    );
    let chinese = structured(
        handle_tool_call(
            &ctx,
            "orchestrator_resolve_category",
            &tool_args(json!({
                "title": "检查这个项目的代码库",
                "description": "先做只读梳理和排查，不要直接改代码"
            })),
        )
        .unwrap(),
    );

    assert_eq!(english["category_id"], "research");
    assert_eq!(english["preferred_role"], "search-specialist");
    assert_eq!(english["requires_subagent_default"], true);
    assert_eq!(chinese["category_id"], "research");
    assert_eq!(chinese["preferred_role"], "search-specialist");
    assert_eq!(chinese["requires_subagent_default"], true);
}

#[test]
fn next_action_for_research_task_requires_parallel_search_subagent() {
    let temp = tempdir().unwrap();
    let plan = temp.path().join("research-plan.md");
    write_file(&plan, research_plan());
    let ctx = create_context(temp.path());

    let payload = structured(
        handle_tool_call(
            &ctx,
            "orchestrator_next_action",
            &tool_args(json!({ "planPath": plan })),
        )
        .unwrap(),
    );

    assert_eq!(payload["task_id"], "R1");
    assert_eq!(payload["action"], "dispatch_task");
    assert_eq!(payload["requires_subagent"], true);
    assert_eq!(payload["dispatch_role"], "search-specialist");
    assert_eq!(payload["dispatch_mode"], "parallel-subagents");
    assert_eq!(payload["task_session_mode"], "spawn-dedicated-task-subagent");
    assert_eq!(payload["task_session_key"], "task::research-plan::R1::implementer");
    assert!(payload["continue_agent_id"].is_null());
    assert_eq!(payload["subagent_tool_action"], "spawn_agent");
    assert_eq!(payload["subagent_agent_type"], "search-specialist");
    assert!(payload["subagent_dispatch_message"]
        .as_str()
        .unwrap()
        .contains("Own top-level task R1"));
    assert_eq!(payload["child_execution_mode"], "current-step");
    assert_eq!(payload["child_execution_label"], "Step 1");
    assert_eq!(
        payload["blocking_control_plane_actions"][0]["tool_name"],
        "orchestrator_begin_task"
    );
    assert_eq!(
        payload["blocking_control_plane_actions"][0]["action"],
        "begin_task"
    );
}

#[test]
fn next_action_batches_dependency_ready_conflict_free_top_level_tasks() {
    let temp = tempdir().unwrap();
    let plan = temp.path().join("parallel-dispatch-plan.md");
    write_file(&plan, parallel_dispatch_plan());
    let ctx = create_context(temp.path());

    let payload = structured(
        handle_tool_call(
            &ctx,
            "orchestrator_next_action",
            &tool_args(json!({ "planPath": plan })),
        )
        .unwrap(),
    );

    let parallel_task_ids = payload["parallel_task_ids"].as_array().unwrap();
    let parallel_dispatches = payload["parallel_dispatches"].as_array().unwrap();
    let plan_state_payload = structured(
        handle_tool_call(
            &ctx,
            "orchestrator_read_plan_state",
            &tool_args(json!({ "planPath": plan })),
        )
        .unwrap(),
    );

    assert_eq!(payload["task_id"], "T1");
    assert_eq!(payload["action"], "acquire_parallel_write_leases");
    assert_eq!(payload["requires_subagent"], false);
    assert_eq!(parallel_task_ids.len(), 2);
    assert_eq!(parallel_task_ids[0], "T1");
    assert_eq!(parallel_task_ids[1], "T2");
    assert_eq!(parallel_dispatches[0]["task_session_mode"], "spawn-dedicated-task-subagent");
    assert_eq!(parallel_dispatches[0]["task_session_key"], "task::parallel-dispatch-plan::T1::implementer");
    assert!(parallel_dispatches[0]["continue_agent_id"].is_null());
    assert_eq!(parallel_dispatches[0]["subagent_tool_action"], "spawn_agent");
    assert_eq!(parallel_dispatches[0]["subagent_agent_type"], "backend-developer");
    assert!(parallel_dispatches[0]["subagent_dispatch_message"]
        .as_str()
        .unwrap()
        .contains("Own top-level task T1"));
    assert_eq!(parallel_dispatches[0]["child_execution_mode"], "current-step");
    assert_eq!(parallel_dispatches[0]["child_execution_label"], "Step 1");
    assert_eq!(
        parallel_dispatches[0]["blocking_control_plane_actions"][0]["tool_name"],
        "orchestrator_begin_task"
    );
    assert_eq!(parallel_dispatches[1]["task_session_mode"], "spawn-dedicated-task-subagent");
    assert_eq!(parallel_dispatches[1]["subagent_tool_action"], "spawn_agent");
    assert_eq!(parallel_dispatches[1]["task_session_key"], "task::parallel-dispatch-plan::T2::implementer");
    assert_eq!(parallel_dispatches[1]["child_execution_mode"], "current-step");
    assert_eq!(
        parallel_dispatches[1]["blocking_control_plane_actions"][0]["tool_name"],
        "orchestrator_begin_task"
    );
    assert_eq!(parallel_dispatches[0]["dispatch_scope"][0], "src/api.rs");
    assert_eq!(parallel_dispatches[1]["dispatch_scope"][0], "src/storage.rs");
    assert_eq!(parallel_dispatches[0]["requires_write_lease"], true);
    assert_eq!(
        plan_state_payload["tasks"][0]["dependsOn"].as_array().unwrap().len(),
        0
    );
    assert_eq!(
        plan_state_payload["tasks"][3]["dependsOn"][0],
        "T1"
    );
    assert_eq!(
        plan_state_payload["tasks"][0]["declaredFiles"][0],
        "src/api.rs"
    );
    assert!(parallel_task_ids.iter().all(|entry| entry != "T3"));
    assert!(parallel_task_ids.iter().all(|entry| entry != "T4"));
}

#[test]
fn next_action_spawns_dedicated_reviewer_session_for_new_review_work() {
    let temp = tempdir().unwrap();
    let plan = temp.path().join("review-lane-plan.md");
    write_file(&plan, review_lane_plan());
    let ctx = create_context(temp.path());
    ctx.runtime_store
        .acquire_write_lease(
            "review-lane-plan",
            "R1",
            "agent-impl",
            &[String::from("src/review.rs")],
        )
        .unwrap();

    let payload = structured(
        handle_tool_call(
            &ctx,
            "orchestrator_next_action",
            &tool_args(json!({ "planPath": plan })),
        )
        .unwrap(),
    );

    assert_eq!(payload["task_id"], "R1");
    assert_eq!(payload["action"], "run_spec_review");
    assert_eq!(payload["task_session_mode"], "spawn-dedicated-reviewer-subagent");
    assert_eq!(payload["task_session_key"], "task::review-lane-plan::R1::review");
    assert!(payload["continue_agent_id"].is_null());
    assert_eq!(payload["subagent_tool_action"], "spawn_agent");
    assert_eq!(payload["subagent_agent_type"], "harness-evaluator");
    assert_eq!(payload["child_execution_mode"], "review-pass");
}

#[test]
fn next_action_resumes_dedicated_task_session_for_repair() {
    let temp = tempdir().unwrap();
    let plan = temp.path().join("repair-lane-plan.md");
    write_file(&plan, repair_lane_plan());
    let ctx = create_context(temp.path());
    ctx.runtime_store
        .acquire_write_lease(
            "repair-lane-plan",
            "Q1",
            "agent-impl",
            &[String::from("src/repair.rs")],
        )
        .unwrap();
    ctx.runtime_store
        .upsert_task_state(TaskStateUpsertInput {
            plan_id: "repair-lane-plan".to_string(),
            task_id: "Q1".to_string(),
            category_id: "backend-impl".to_string(),
            status: "spec_failed".to_string(),
            active_step_label: None,
            assigned_role: Some("backend-developer".to_string()),
            agent_id: Some("agent-review".to_string()),
            implementation_agent_id: Some("agent-impl".to_string()),
            review_agent_id: Some("agent-review".to_string()),
            write_lease_id: None,
            spec_review_status: "fail".to_string(),
            quality_review_status: "pending".to_string(),
            retry_count: 0,
            blocker_type: None,
            blocker_message: None,
        })
        .unwrap();

    let payload = structured(
        handle_tool_call(
            &ctx,
            "orchestrator_next_action",
            &tool_args(json!({ "planPath": plan })),
        )
        .unwrap(),
    );

    assert_eq!(payload["task_id"], "Q1");
    assert_eq!(payload["action"], "return_to_implementer");
    assert_eq!(payload["task_session_mode"], "resume-dedicated-task-subagent");
    assert_eq!(payload["task_session_key"], "task::repair-lane-plan::Q1::implementer");
    assert_eq!(payload["continue_agent_id"], "agent-impl");
    assert_eq!(payload["subagent_tool_action"], "send_input");
    assert_eq!(payload["subagent_agent_type"], "backend-developer");
    assert_eq!(payload["child_execution_mode"], "current-step");
}

#[test]
fn next_action_requires_step_sync_checkpoint_before_resuming_running_task() {
    let temp = tempdir().unwrap();
    let plan = temp.path().join("step-sync-repair-plan.md");
    write_file(&plan, step_sync_repair_plan());
    let ctx = create_context(temp.path());
    ctx.runtime_store
        .acquire_write_lease(
            "step-sync-repair-plan",
            "S1",
            "agent-impl",
            &[String::from("src/step.rs")],
        )
        .unwrap();
    ctx.runtime_store
        .upsert_task_state(TaskStateUpsertInput {
            plan_id: "step-sync-repair-plan".to_string(),
            task_id: "S1".to_string(),
            category_id: "backend-impl".to_string(),
            status: "running_impl".to_string(),
            active_step_label: None,
            assigned_role: Some("backend-developer".to_string()),
            agent_id: Some("agent-impl".to_string()),
            implementation_agent_id: Some("agent-impl".to_string()),
            review_agent_id: None,
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
            "orchestrator_next_action",
            &tool_args(json!({ "planPath": plan })),
        )
        .unwrap(),
    );

    assert_eq!(payload["task_id"], "S1");
    assert_eq!(payload["action"], "continue_same_agent");
    assert_eq!(payload["subagent_tool_action"], "send_input");
    assert_eq!(payload["subagent_agent_type"], "backend-developer");
    assert_eq!(payload["child_execution_mode"], "current-step");
    assert_eq!(
        payload["blocking_control_plane_actions"][0]["tool_name"],
        "orchestrator_begin_step"
    );
    assert_eq!(
        payload["blocking_control_plane_actions"][0]["action"],
        "begin_step"
    );
    assert_eq!(
        payload["blocking_control_plane_actions"][0]["step_label"],
        "Step 1"
    );
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
            implementation_agent_id: Some("agent-impl".to_string()),
            review_agent_id: None,
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
fn question_gate_skips_redundant_direction_confirmation() {
    let temp = tempdir().unwrap();
    let ctx = create_context(temp.path());

    let payload = structured(
        handle_tool_call(
            &ctx,
            "orchestrator_question_gate",
            &tool_args(json!({
                "questionCategory": "direction_confirmation",
                "reason": "The user already requested a full MCP server and there is no hard blocker."
            })),
        )
        .unwrap(),
    );

    assert_eq!(payload["ask_user"], false);
    assert_eq!(payload["allowed_to_expand"], false);
    assert_eq!(payload["recommended_action"], "plan_and_execute");
}

#[test]
fn terminal_quality_review_pass_immediately_accepts_and_advances_todo_mirror() {
    let temp = tempdir().unwrap();
    let plan = temp.path().join("immediate-acceptance-plan.md");
    write_file(&plan, immediate_acceptance_plan());
    let ctx = create_context(temp.path());
    ctx.runtime_store
        .upsert_plan_state(codex_orchestrator_mcp::runtime_store::PlanStateUpsertInput {
            plan_id: "immediate-acceptance-plan".to_string(),
            plan_path: plan.to_str().unwrap().to_string(),
            spec_path: None,
            current_wave: Some("Wave Accept".to_string()),
            active_task_id: Some("A1".to_string()),
            last_review_result: None,
        })
        .unwrap();
    ctx.runtime_store
        .upsert_task_state(TaskStateUpsertInput {
            plan_id: "immediate-acceptance-plan".to_string(),
            task_id: "A1".to_string(),
            category_id: "backend-impl".to_string(),
            status: "running_quality_review".to_string(),
            active_step_label: None,
            assigned_role: Some("backend-developer".to_string()),
            agent_id: Some("agent-impl".to_string()),
            implementation_agent_id: Some("agent-impl".to_string()),
            review_agent_id: Some("agent-review".to_string()),
            write_lease_id: None,
            spec_review_status: "pass".to_string(),
            quality_review_status: "pending".to_string(),
            retry_count: 0,
            blocker_type: None,
            blocker_message: None,
        })
        .unwrap();

    let review_payload = structured(
        handle_tool_call(
            &ctx,
            "orchestrator_record_review",
            &tool_args(json!({
                "planPath": plan,
                "taskId": "A1",
                "reviewType": "quality",
                "result": "pass",
                "reviewerAgentId": "agent-review"
            })),
        )
        .unwrap(),
    );

    assert_eq!(review_payload["task_status"], "accepted");
    assert_eq!(review_payload["accepted"], true);
    assert_eq!(review_payload["top_level_todo_checked"], true);
    assert_eq!(review_payload["next_active_task_id"], "B1");

    let todo_payload = structured(
        handle_tool_call(
            &ctx,
            "orchestrator_export_codex_todo",
            &tool_args(json!({ "planPath": plan })),
        )
        .unwrap(),
    );
    let items = todo_payload["items"].as_array().unwrap();
    assert_eq!(todo_payload["active_task_id"], "B1");
    assert_eq!(items[0]["status"], "completed");
    assert_eq!(items[1]["status"], "in_progress");

    let plan_state_payload = structured(
        handle_tool_call(
            &ctx,
            "orchestrator_read_plan_state",
            &tool_args(json!({ "planPath": plan })),
        )
        .unwrap(),
    );
    assert_eq!(plan_state_payload["executionStatus"]["activeTask"], "B1");
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
            implementation_agent_id: Some("agent-impl".to_string()),
            review_agent_id: None,
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
