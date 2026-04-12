use std::{
    fs,
    path::{Path, PathBuf},
};

use regex::Regex;
use serde_json::Value as JsonValue;
use toml::Value as TomlValue;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(3)
        .expect("rust-cli manifest should be nested under the repository root")
        .to_path_buf()
}

fn read_file(path: &Path) -> String {
    fs::read_to_string(path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()))
}

fn read_json(path: &Path) -> JsonValue {
    serde_json::from_str(&read_file(path))
        .unwrap_or_else(|err| panic!("failed to parse json {}: {err}", path.display()))
}

fn read_toml(path: &Path) -> TomlValue {
    toml::from_str(&read_file(path))
        .unwrap_or_else(|err| panic!("failed to parse toml {}: {err}", path.display()))
}

fn collect_markdown_files(start: &Path, output: &mut Vec<PathBuf>) {
    let metadata = fs::metadata(start)
        .unwrap_or_else(|err| panic!("failed to stat {}: {err}", start.display()));
    if metadata.is_file() {
        if start
            .extension()
            .and_then(|ext| ext.to_str())
            .is_some_and(|ext| ext.eq_ignore_ascii_case("md"))
        {
            output.push(start.to_path_buf());
        }
        return;
    }

    for entry in fs::read_dir(start)
        .unwrap_or_else(|err| panic!("failed to read dir {}: {err}", start.display()))
    {
        let entry = entry.unwrap();
        let path = entry.path();
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if entry.file_type().unwrap().is_dir() {
            if name == ".git" || name == "target" || name == "node_modules" {
                continue;
            }
            collect_markdown_files(&path, output);
        } else if path
            .extension()
            .and_then(|ext| ext.to_str())
            .is_some_and(|ext| ext.eq_ignore_ascii_case("md"))
        {
            output.push(path);
        }
    }
}

fn collect_typescript_files(start: &Path, output: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(start)
        .unwrap_or_else(|err| panic!("failed to read dir {}: {err}", start.display()))
    {
        let entry = entry.unwrap();
        let path = entry.path();
        let name = entry.file_name();
        let name = name.to_string_lossy();
        let file_type = entry.file_type().unwrap();
        if file_type.is_dir() {
            if name == ".git" || name == "target" || name == "node_modules" {
                continue;
            }
            collect_typescript_files(&path, output);
            continue;
        }

        let is_ts = path
            .extension()
            .and_then(|ext| ext.to_str())
            .is_some_and(|ext| ext.eq_ignore_ascii_case("ts") || ext.eq_ignore_ascii_case("tsx"));
        if is_ts {
            output.push(path);
        }
    }
}

fn assert_matches(haystack: &str, pattern: &str, context: &str) {
    let regex = Regex::new(pattern).unwrap();
    assert!(
        regex.is_match(haystack),
        "{context} did not match pattern `{pattern}`.\nInput:\n{haystack}"
    );
}

#[test]
fn bundled_agent_inventory_and_roles_stay_aligned() {
    let repo = repo_root();
    let plugin_root = repo.join("plugins").join("codex-orchestrator");
    let bundle_dir = plugin_root.join("codex").join("agents");
    let openai_manifest_path = plugin_root.join("agents").join("openai.yaml");
    let categories_path = plugin_root.join("config").join("categories.toml");
    let expected_bundle = [
        "harness-planner",
        "harness-dispatch-gate",
        "search-specialist",
        "backend-developer",
        "harness-evaluator",
        "harness-doc-gardener",
    ];

    let manifest = read_file(&openai_manifest_path);
    assert_matches(&manifest, r#"path:\s+"\.\./codex/agents/""#, "openai agent manifest");
    assert_matches(&manifest, r#"format:\s+"toml""#, "openai agent manifest");

    for agent in expected_bundle {
        assert!(
            bundle_dir.join(format!("{agent}.toml")).exists(),
            "missing bundled agent file for {agent}"
        );
        assert_matches(
            &manifest,
            &format!(r"- {}\b", regex::escape(agent)),
            "openai agent manifest",
        );
    }

    let categories = read_toml(&categories_path);
    let preferred_roles = ["plan", "research", "backend-impl", "review"].map(|section| {
        categories
            .get(section)
            .and_then(TomlValue::as_table)
            .and_then(|table| table.get("preferred_role"))
            .and_then(TomlValue::as_str)
            .unwrap_or_else(|| panic!("missing preferred_role for section {section}"))
            .to_string()
    });

    for role in preferred_roles {
        assert!(
            expected_bundle.contains(&role.as_str()),
            "preferred role {role} must be present in the bundled agent set"
        );
    }

    let backend = read_file(&bundle_dir.join("backend-developer.toml"));
    let planner = read_file(&bundle_dir.join("harness-planner.toml"));
    let evaluator = read_file(&bundle_dir.join("harness-evaluator.toml"));
    let search = read_file(&bundle_dir.join("search-specialist.toml"));

    assert_matches(&backend, r"<intent_gate>", "backend-developer.toml");
    assert_matches(&backend, r"<verification_loop>", "backend-developer.toml");
    assert_matches(&backend, r"<completion_contract>", "backend-developer.toml");
    assert_matches(&planner, r"<intent_gate>", "harness-planner.toml");
    assert_matches(&planner, r"<planning_contract>", "harness-planner.toml");
    assert_matches(&evaluator, r"findings-first", "harness-evaluator.toml");
    assert_matches(&evaluator, r"<completion_contract>", "harness-evaluator.toml");
    assert_matches(&search, r"<search_rules>", "search-specialist.toml");
    assert_matches(&search, r"<return_contract>", "search-specialist.toml");
}

#[test]
fn orchestrator_workflow_absorbs_brainstorming_contract() {
    let repo = repo_root();
    let plugin_root = repo.join("plugins").join("codex-orchestrator");
    let skill = read_file(&plugin_root.join("skills").join("orchestrator").join("SKILL.md"));
    let repo_agents = read_file(&repo.join("AGENTS.md"));
    let install_guide = read_file(&repo.join("install.md"));
    let manifest = read_json(&plugin_root.join(".codex-plugin").join("plugin.json"));
    let default_prompt = manifest["interface"]["defaultPrompt"]
        .as_array()
        .expect("plugin manifest defaultPrompt must be an array")
        .iter()
        .filter_map(|value| value.as_str())
        .collect::<Vec<_>>()
        .join("\n");

    assert_matches(
        &skill,
        r"do not enter through `using-superpowers` or standalone `brainstorming`",
        "orchestrator skill",
    );
    assert_matches(
        &skill,
        r"ask clarifying questions one at a time",
        "orchestrator skill",
    );
    assert_matches(
        &skill,
        r"direction is materially open or the user explicitly asked for options",
        "orchestrator skill",
    );
    assert_matches(
        &skill,
        r"user already supplied a workable direction",
        "orchestrator skill",
    );
    assert_matches(&skill, r"spec self-review", "orchestrator skill");
    assert_matches(
        &skill,
        r"without asking a second confirmation question",
        "orchestrator skill",
    );

    assert_matches(
        &repo_agents,
        r"must not enter through `using-superpowers` or standalone `brainstorming`",
        "repo AGENTS",
    );
    assert_matches(
        &repo_agents,
        r"compare 2-3 approaches only when the direction is still open",
        "repo AGENTS",
    );
    assert_matches(
        &install_guide,
        r"must not enter through `using-superpowers` or standalone `brainstorming`",
        "install guide",
    );
    assert_matches(
        &install_guide,
        r"do not ask a second confirmation question",
        "install guide",
    );

    assert_matches(
        manifest["description"].as_str().unwrap(),
        r"discovery, direction-aware planning, file-backed execution tracking",
        "plugin description",
    );
    assert_matches(
        manifest["interface"]["shortDescription"].as_str().unwrap(),
        r"discovery, direction-aware planning, execution",
        "plugin short description",
    );
    assert_matches(
        manifest["interface"]["longDescription"].as_str().unwrap(),
        r"direction-aware specs and implementation plans",
        "plugin long description",
    );
    assert_matches(
        &default_prompt,
        r"Clarify only hard blockers",
        "plugin default prompts",
    );
    assert_matches(
        &default_prompt,
        r"do not ask again to start",
        "plugin default prompts",
    );
}

#[test]
fn inspection_first_workflow_routes_repo_checks_to_search_specialist() {
    let repo = repo_root();
    let plugin_root = repo.join("plugins").join("codex-orchestrator");
    let skill = read_file(&plugin_root.join("skills").join("orchestrator").join("SKILL.md"));
    let repo_agents = read_file(&repo.join("AGENTS.md"));
    let install_guide = read_file(&repo.join("install.md"));

    assert_matches(
        &skill,
        r"codebase checks, repo audits, and read-only repo-understanding requests as `research` work",
        "orchestrator skill",
    );
    assert_matches(
        &skill,
        r"dispatching `search-specialist`",
        "orchestrator skill",
    );
    assert_matches(
        &repo_agents,
        r"codebase-check, repo-audit, and read-only codebase-understanding requests",
        "repo AGENTS",
    );
    assert_matches(
        &repo_agents,
        r"Do not let the parent absorb first-pass repo inspection",
        "repo AGENTS",
    );
    assert_matches(
        &install_guide,
        r"codebase-check, repo-audit, and read-only repo-understanding requests",
        "install guide",
    );
    assert_matches(&install_guide, r"dispatch `search-specialist`", "install guide");
}

#[test]
fn workflow_requires_immediate_top_level_acceptance_after_terminal_review() {
    let repo = repo_root();
    let plugin_root = repo.join("plugins").join("codex-orchestrator");
    let skill = read_file(&plugin_root.join("skills").join("orchestrator").join("SKILL.md"));
    let repo_agents = read_file(&repo.join("AGENTS.md"));
    let install_guide = read_file(&repo.join("install.md"));
    let agent_contracts = read_file(&repo.join("docs").join("architecture").join("agent-contracts.md"));
    let mcp_contract = read_file(&repo.join("docs").join("architecture").join("mcp-tool-contract.md"));
    let plan_sync = read_file(&repo.join("docs").join("architecture").join("plan-sync-rules.md"));

    assert_matches(
        &skill,
        r"terminal review pass closes the task, accept the top-level task in the same control-plane pass",
        "orchestrator skill",
    );
    assert_matches(
        &repo_agents,
        r"terminal review pass closes a task, parent acceptance must happen in the same control-plane pass",
        "repo AGENTS",
    );
    assert_matches(
        &install_guide,
        r"terminal review pass closes a task, accept the top-level task in the same control-plane pass",
        "install guide",
    );
    assert_matches(
        &agent_contracts,
        r"terminal review pass closes a task, the parent must accept that top-level task in the same control-plane pass",
        "agent contracts",
    );
    assert_matches(
        &mcp_contract,
        r"`record_review` immediately accepts a terminal-ready task",
        "mcp tool contract",
    );
    assert_matches(
        &plan_sync,
        r"terminal review pass closes a task, accept it in the same control-plane pass",
        "plan sync rules",
    );
}

#[test]
fn workflow_requires_parallel_dispatch_for_dependency_ready_conflict_free_tasks() {
    let repo = repo_root();
    let plugin_root = repo.join("plugins").join("codex-orchestrator");
    let skill = read_file(&plugin_root.join("skills").join("orchestrator").join("SKILL.md"));
    let repo_agents = read_file(&repo.join("AGENTS.md"));
    let install_guide = read_file(&repo.join("install.md"));
    let category_contract = read_file(&repo.join("docs").join("architecture").join("category-contract.md"));
    let agent_contracts = read_file(&repo.join("docs").join("architecture").join("agent-contracts.md"));
    let mcp_contract = read_file(&repo.join("docs").join("architecture").join("mcp-tool-contract.md"));

    assert_matches(
        &skill,
        r"parallel_task_ids` and `parallel_dispatches`",
        "orchestrator skill",
    );
    assert_matches(
        &skill,
        r"acquire one lease per returned child dispatch scope",
        "orchestrator skill",
    );
    assert_matches(
        &repo_agents,
        r"dispatch them together as one parallel child batch",
        "repo AGENTS",
    );
    assert_matches(
        &repo_agents,
        r"first task id only as the native todo mirror anchor",
        "repo AGENTS",
    );
    assert_matches(
        &install_guide,
        r"parallel top-level dispatch",
        "install guide",
    );
    assert_matches(
        &install_guide,
        r"launch the whole returned cohort in one round",
        "install guide",
    );
    assert_matches(
        &category_contract,
        r"dependency-ready top-level tasks share a category whose parallelism contract permits batching",
        "category contract",
    );
    assert_matches(
        &agent_contracts,
        r"`parallel_task_ids`",
        "agent contracts",
    );
    assert_matches(
        &agent_contracts,
        r"`parallel_dispatches`",
        "agent contracts",
    );
    assert_matches(
        &mcp_contract,
        r"exposes a parallel dispatch cohort",
        "mcp tool contract",
    );
}

#[test]
fn workflow_requires_task_owned_subagent_sessions() {
    let repo = repo_root();
    let plugin_root = repo.join("plugins").join("codex-orchestrator");
    let skill = read_file(&plugin_root.join("skills").join("orchestrator").join("SKILL.md"));
    let repo_agents = read_file(&repo.join("AGENTS.md"));
    let install_guide = read_file(&repo.join("install.md"));
    let runtime_schema = read_file(&repo.join("docs").join("architecture").join("runtime-state-schema.md"));
    let agent_contracts = read_file(&repo.join("docs").join("architecture").join("agent-contracts.md"));
    let mcp_contract = read_file(&repo.join("docs").join("architecture").join("mcp-tool-contract.md"));

    assert_matches(
        &skill,
        r"dedicated child session",
        "orchestrator skill",
    );
    assert_matches(
        &skill,
        r"`task_session_mode`, `task_session_key`, and `continue_agent_id`",
        "orchestrator skill",
    );
    assert_matches(
        &repo_agents,
        r"Give each top-level task its own dedicated child session",
        "repo AGENTS",
    );
    assert_matches(
        &install_guide,
        r"task-owned child sessions",
        "install guide",
    );
    assert_matches(
        &runtime_schema,
        r"`implementation_agent_id`",
        "runtime state schema",
    );
    assert_matches(
        &runtime_schema,
        r"`review_agent_id`",
        "runtime state schema",
    );
    assert_matches(
        &agent_contracts,
        r"`task_session_mode`",
        "agent contracts",
    );
    assert_matches(
        &agent_contracts,
        r"One top-level task should map to one dedicated implementer child session",
        "agent contracts",
    );
    assert_matches(
        &mcp_contract,
        r"`task_session_key`",
        "mcp tool contract",
    );
    assert_matches(
        &mcp_contract,
        r"one dedicated child per top-level task",
        "mcp tool contract",
    );
}

#[test]
fn workflow_requires_executable_subagent_dispatch_contract() {
    let repo = repo_root();
    let plugin_root = repo.join("plugins").join("codex-orchestrator");
    let skill = read_file(&plugin_root.join("skills").join("orchestrator").join("SKILL.md"));
    let repo_agents = read_file(&repo.join("AGENTS.md"));
    let install_guide = read_file(&repo.join("install.md"));
    let agent_contracts = read_file(&repo.join("docs").join("architecture").join("agent-contracts.md"));
    let mcp_contract = read_file(&repo.join("docs").join("architecture").join("mcp-tool-contract.md"));

    assert_matches(
        &skill,
        r"`subagent_tool_action`, `subagent_agent_type`, and `subagent_dispatch_message`",
        "orchestrator skill",
    );
    assert_matches(&skill, r"`spawn_agent`", "orchestrator skill");
    assert_matches(&skill, r"`send_input`", "orchestrator skill");
    assert_matches(
        &repo_agents,
        r"`spawn_agent` or `send_input`",
        "repo AGENTS",
    );
    assert_matches(
        &install_guide,
        r"`subagent_tool_action`",
        "install guide",
    );
    assert_matches(
        &agent_contracts,
        r"`subagent_dispatch_message`",
        "agent contracts",
    );
    assert_matches(
        &mcp_contract,
        r"`subagent_tool_action`",
        "mcp tool contract",
    );
}

#[test]
fn workflow_requires_mid_run_control_plane_checkpoints() {
    let repo = repo_root();
    let plugin_root = repo.join("plugins").join("codex-orchestrator");
    let skill = read_file(&plugin_root.join("skills").join("orchestrator").join("SKILL.md"));
    let repo_agents = read_file(&repo.join("AGENTS.md"));
    let install_guide = read_file(&repo.join("install.md"));
    let plan_sync = read_file(&repo.join("docs").join("architecture").join("plan-sync-rules.md"));
    let agent_contracts = read_file(&repo.join("docs").join("architecture").join("agent-contracts.md"));
    let mcp_contract = read_file(&repo.join("docs").join("architecture").join("mcp-tool-contract.md"));
    let backend_agent = read_file(&plugin_root.join("codex").join("agents").join("backend-developer.toml"));
    let search_agent = read_file(&plugin_root.join("codex").join("agents").join("search-specialist.toml"));
    let planner_agent = read_file(&plugin_root.join("codex").join("agents").join("harness-planner.toml"));

    assert_matches(
        &skill,
        r"blocking control-plane actions",
        "orchestrator skill",
    );
    assert_matches(
        &skill,
        r"only the current step on that resume",
        "orchestrator skill",
    );
    assert_matches(
        &repo_agents,
        r"must perform those writes before launching or resuming the child",
        "repo AGENTS",
    );
    assert_matches(
        &install_guide,
        r"`blocking_control_plane_actions`",
        "install guide",
    );
    assert_matches(
        &plan_sync,
        r"must not be deferred to a terminal replay batch",
        "plan sync rules",
    );
    assert_matches(
        &agent_contracts,
        r"`blocking_control_plane_actions`",
        "agent contracts",
    );
    assert_matches(
        &mcp_contract,
        r"`child_execution_mode`",
        "mcp tool contract",
    );
    assert_matches(
        &backend_agent,
        r"only the current step for that task resume",
        "backend bundled agent",
    );
    assert_matches(
        &search_agent,
        r"only the current step for that task resume",
        "search bundled agent",
    );
    assert_matches(
        &planner_agent,
        r"only the current step for that task resume",
        "planner bundled agent",
    );
}

#[test]
fn markdown_docs_do_not_contain_machine_specific_absolute_paths() {
    let repo = repo_root();
    let targets = [
        repo.join("AGENTS.md"),
        repo.join("README.md"),
        repo.join("install.md"),
        repo.join("docs"),
        repo.join("plugins").join("codex-orchestrator").join("skills"),
    ];
    let forbidden = Regex::new(r#"(/Users/|/home/|/mnt/|/tmp/|[A-Za-z]:\\)"#).unwrap();
    let mut markdown_files = Vec::new();
    let mut findings = Vec::new();

    for target in targets {
        collect_markdown_files(&target, &mut markdown_files);
    }

    for file_path in markdown_files {
        let contents = read_file(&file_path);
        for (index, line) in contents.lines().enumerate() {
            if forbidden.is_match(line) {
                findings.push(format!(
                    "{}:{}:{}",
                    file_path.strip_prefix(&repo).unwrap().display(),
                    index + 1,
                    line.trim()
                ));
            }
        }
    }

    assert!(
        findings.is_empty(),
        "found machine-specific absolute paths in markdown docs:\n{}",
        findings.join("\n")
    );
}

#[test]
fn plugin_manifest_uses_repository_backed_metadata() {
    let repo = repo_root();
    let plugin_root = repo.join("plugins").join("codex-orchestrator");
    let manifest = read_json(&plugin_root.join(".codex-plugin").join("plugin.json"));
    let repo_url = "https://github.com/Viking602/codex-orchestrator";
    let privacy_doc = repo.join("docs").join("product").join("privacy-policy.md");
    let terms_doc = repo.join("docs").join("product").join("terms-of-service.md");

    assert!(privacy_doc.exists(), "privacy policy doc should exist");
    assert!(terms_doc.exists(), "terms doc should exist");
    assert_eq!(manifest["author"]["name"].as_str().unwrap(), "Viking");
    assert_eq!(
        manifest["author"]["email"].as_str().unwrap(),
        "chen17090314747@gmail.com"
    );
    assert_eq!(
        manifest["author"]["url"].as_str().unwrap(),
        "https://github.com/Viking602"
    );
    assert_eq!(manifest["homepage"].as_str().unwrap(), repo_url);
    assert_eq!(manifest["repository"].as_str().unwrap(), repo_url);
    assert_eq!(manifest["interface"]["websiteURL"].as_str().unwrap(), repo_url);
    assert_eq!(
        manifest["interface"]["privacyPolicyURL"].as_str().unwrap(),
        format!("{repo_url}/blob/master/docs/product/privacy-policy.md")
    );
    assert_eq!(
        manifest["interface"]["termsOfServiceURL"].as_str().unwrap(),
        format!("{repo_url}/blob/master/docs/product/terms-of-service.md")
    );

    let manifest_json = serde_json::to_string(&manifest).unwrap();
    assert!(
        !manifest_json.contains("example.invalid"),
        "scaffold placeholder domains must not ship in the manifest"
    );
}

#[test]
fn active_repository_surface_contains_no_typescript_files() {
    let repo = repo_root();
    let plugin_root = repo.join("plugins").join("codex-orchestrator");
    let plugin_package = read_json(&plugin_root.join("package.json"));
    let scripts = plugin_package["scripts"]
        .as_object()
        .expect("plugin package scripts must be present");
    let mut ts_files = Vec::new();

    collect_typescript_files(&repo, &mut ts_files);

    assert!(
        ts_files.is_empty(),
        "expected no .ts or .tsx files in the repository, found:\n{}",
        ts_files
            .iter()
            .map(|path| path.strip_prefix(&repo).unwrap().display().to_string())
            .collect::<Vec<_>>()
            .join("\n")
    );

    for script_name in ["test", "serve"] {
        let script = scripts
            .get(script_name)
            .and_then(JsonValue::as_str)
            .unwrap_or_default();
        assert!(
            !Regex::new(r#"node\s+--experimental-strip-types|src/server\.ts|tests/.*\.test\.ts"#)
                .unwrap()
                .is_match(script),
            "plugin package script `{script_name}` still advertises deleted TypeScript paths: {script}"
        );
    }
}
