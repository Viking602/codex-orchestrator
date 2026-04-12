use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DocDriftResult {
    pub update_agents: bool,
    pub update_docs_index: bool,
    pub update_architecture_docs: bool,
    pub update_product_docs: bool,
    pub reasons: Vec<String>,
}

pub fn check_doc_drift(changed_paths: &[String]) -> DocDriftResult {
    let normalized: Vec<String> = changed_paths
        .iter()
        .map(|entry| entry.replace('\\', "/"))
        .collect();
    let touches_plugin_code = normalized
        .iter()
        .any(|entry| entry.starts_with("plugins/codex-orchestrator/"));
    let touches_architecture = normalized
        .iter()
        .any(|entry| entry.starts_with("docs/architecture/"));
    let touches_product = normalized
        .iter()
        .any(|entry| entry.starts_with("docs/product/"));
    let touches_routing = normalized
        .iter()
        .any(|entry| entry == "AGENTS.md" || entry == "docs/index.md");

    let mut reasons = Vec::new();
    if touches_plugin_code {
        reasons.push("plugin surface changed".to_string());
    }
    if touches_architecture {
        reasons.push("architecture docs already touched".to_string());
    }
    if touches_product {
        reasons.push("product docs already touched".to_string());
    }
    if touches_routing {
        reasons.push("routing docs already touched".to_string());
    }

    DocDriftResult {
        update_agents: touches_plugin_code && !normalized.iter().any(|entry| entry == "AGENTS.md"),
        update_docs_index: touches_plugin_code
            && !normalized.iter().any(|entry| entry == "docs/index.md"),
        update_architecture_docs: touches_plugin_code && !touches_architecture,
        update_product_docs: false,
        reasons: if reasons.is_empty() {
            vec!["no drift detected".to_string()]
        } else {
            reasons
        },
    }
}
