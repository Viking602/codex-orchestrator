use std::{collections::HashMap, fs};

use anyhow::{anyhow, Context, Result};
use serde::Deserialize;

use crate::types::{CategoryDefinition, CategoryResolution, DelegationPreference};

#[derive(Debug, Deserialize)]
struct RawCategory {
    intent: Option<String>,
    preferred_role: Option<String>,
    allowed_roles: Option<Vec<String>>,
    write_policy: Option<String>,
    requires_plan: Option<bool>,
    requires_spec_review: Option<bool>,
    requires_quality_review: Option<bool>,
    parallelism: Option<String>,
    delegation_preference: Option<String>,
    reuse_policy: Option<String>,
    completion_contract: Option<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct CategoryRegistry {
    categories: HashMap<String, CategoryDefinition>,
}

impl CategoryRegistry {
    pub fn from_toml(file_path: &str) -> Result<Self> {
        let source = fs::read_to_string(file_path)
            .with_context(|| format!("failed to read categories TOML: {file_path}"))?;
        let parsed: HashMap<String, RawCategory> =
            toml::from_str(&source).context("failed to parse categories TOML")?;

        let definitions = parsed
            .into_iter()
            .map(|(id, raw)| {
                let preferred_role = raw.preferred_role.unwrap_or_else(|| "default".to_string());
                let allowed_roles = raw
                    .allowed_roles
                    .unwrap_or_else(|| vec![preferred_role.clone()]);
                Ok(CategoryDefinition {
                    id: id.clone(),
                    intent: raw.intent.unwrap_or_else(|| id.clone()),
                    preferred_role,
                    allowed_roles,
                    write_policy: raw.write_policy.unwrap_or_else(|| "read-only".to_string()),
                    requires_plan: raw.requires_plan.unwrap_or(false),
                    requires_spec_review: raw.requires_spec_review.unwrap_or(false),
                    requires_quality_review: raw.requires_quality_review.unwrap_or(false),
                    parallelism: raw.parallelism.unwrap_or_else(|| "single".to_string()),
                    delegation_preference: normalize_delegation_preference(
                        &id,
                        raw.delegation_preference.as_deref(),
                    )?,
                    reuse_policy: raw.reuse_policy.unwrap_or_else(|| "no_reuse".to_string()),
                    completion_contract: raw.completion_contract.unwrap_or_default(),
                })
            })
            .collect::<Result<Vec<_>>>()?;

        let categories = definitions
            .into_iter()
            .map(|definition| (definition.id.clone(), definition))
            .collect();

        Ok(Self { categories })
    }

    pub fn get(&self, category_id: &str) -> Option<&CategoryDefinition> {
        self.categories.get(category_id)
    }

    pub fn list(&self) -> Vec<CategoryDefinition> {
        self.categories.values().cloned().collect()
    }

    pub fn resolve(
        &self,
        title: &str,
        description: &str,
        explicit_category: Option<&str>,
    ) -> Result<CategoryResolution> {
        if let Some(explicit_category) = explicit_category {
            let explicit = self
                .get(explicit_category)
                .cloned()
                .ok_or_else(|| anyhow!("Unknown category: {explicit_category}"))?;
            return Ok(CategoryResolution {
                category_id: explicit.id.clone(),
                reason: "explicit_category".to_string(),
                category: explicit,
            });
        }

        let haystack = format!("{title}\n{description}").to_lowercase();

        let planners = ["plan", "spec", "design", "architecture", "contract", "decompose"];
        if planners.iter().any(|term| haystack.contains(term)) {
            let category = self.require("plan")?;
            return Ok(CategoryResolution {
                category_id: category.id.clone(),
                reason: "planning_keywords".to_string(),
                category,
            });
        }

        let reviewers = ["review", "verify", "evaluation", "evaluator", "qa", "quality"];
        if reviewers.iter().any(|term| haystack.contains(term)) {
            let category = self.require("review")?;
            return Ok(CategoryResolution {
                category_id: category.id.clone(),
                reason: "review_keywords".to_string(),
                category,
            });
        }

        let researchers = [
            "research",
            "analyze",
            "analysis",
            "inspect",
            "scan",
            "find",
            "investigate",
            "map",
        ];
        if researchers.iter().any(|term| haystack.contains(term)) {
            let category = self.require("research")?;
            return Ok(CategoryResolution {
                category_id: category.id.clone(),
                reason: "research_keywords".to_string(),
                category,
            });
        }

        let category = self.require("backend-impl")?;
        Ok(CategoryResolution {
            category_id: category.id.clone(),
            reason: "default_backend_impl".to_string(),
            category,
        })
    }

    fn require(&self, category_id: &str) -> Result<CategoryDefinition> {
        self.get(category_id)
            .cloned()
            .ok_or_else(|| anyhow!("Missing required category definition: {category_id}"))
    }
}

fn normalize_delegation_preference(
    category_id: &str,
    value: Option<&str>,
) -> Result<DelegationPreference> {
    match value {
        Some("parent-only") => Ok(DelegationPreference::ParentOnly),
        Some("prefer-subagent") => Ok(DelegationPreference::PreferSubagent),
        Some("subagent-required") => Ok(DelegationPreference::SubagentRequired),
        Some(other) => Err(anyhow!(
            "Category {category_id} has invalid delegation_preference: {other}"
        )),
        None => Err(anyhow!(
            "Category {category_id} is missing required delegation_preference"
        )),
    }
}
