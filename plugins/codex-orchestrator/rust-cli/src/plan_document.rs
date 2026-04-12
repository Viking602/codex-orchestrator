use std::{
    fs,
    path::Path,
};

use anyhow::{anyhow, Context, Result};
use regex::Regex;
use serde::Serialize;

use crate::types::{FinalAcceptanceItem, PlanExecutionStatus, PlanStep, PlanTask};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PlanState {
    pub execution_status: PlanExecutionStatus,
    pub tasks: Vec<PlanTask>,
}

#[derive(Debug, Clone)]
pub struct PlanDocument {
    current_plan_path: String,
}

impl PlanDocument {
    pub fn new(plan_path: &str) -> Self {
        Self {
            current_plan_path: resolve_initial_plan_path(plan_path),
        }
    }

    pub fn plan_path(&self) -> &str {
        &self.current_plan_path
    }

    pub fn read_text(&mut self) -> Result<String> {
        self.ensure_archived_if_completed()?;
        Ok(normalize_line_endings(&fs::read_to_string(&self.current_plan_path).with_context(
            || format!("failed to read plan file: {}", self.current_plan_path),
        )?))
    }

    pub fn read_lines(&mut self) -> Result<Vec<String>> {
        Ok(self
            .read_text()?
            .split('\n')
            .map(|line| line.to_string())
            .collect())
    }

    pub fn read_plan_state(&mut self) -> Result<PlanState> {
        let lines = self.read_lines()?;
        Ok(PlanState {
            execution_status: parse_execution_status(&lines)?,
            tasks: parse_tasks(&lines)?,
        })
    }

    pub fn update_execution_status(
        &mut self,
        current_wave: Option<&str>,
        active_task: Option<&str>,
        blockers: Option<&str>,
        last_review_result: Option<&str>,
    ) -> Result<()> {
        let mut lines = self.read_lines()?;
        let range = find_section_range(&lines, "## Execution Status")
            .ok_or_else(|| anyhow!("Execution Status section not found"))?;

        update_prefixed_line(&mut lines, &range, "- Current wave:", current_wave)?;
        update_prefixed_line(&mut lines, &range, "- Active task:", active_task)?;
        update_prefixed_line(&mut lines, &range, "- Blockers:", blockers)?;
        update_prefixed_line(
            &mut lines,
            &range,
            "- Last review result:",
            last_review_result,
        )?;

        self.write_lines(lines)
    }

    pub fn update_task_metadata(
        &mut self,
        task_id: &str,
        task_status: Option<&str>,
        current_step: Option<&str>,
        spec_review_status: Option<&str>,
        quality_review_status: Option<&str>,
        assigned_agent: Option<&str>,
    ) -> Result<()> {
        let mut lines = self.read_lines()?;
        let range =
            find_task_range(&lines, task_id).ok_or_else(|| anyhow!("Task block not found: {task_id}"))?;

        update_prefixed_line(&mut lines, &range, "**Task Status:**", task_status)?;
        update_prefixed_line(&mut lines, &range, "**Current Step:**", current_step)?;
        update_prefixed_line(
            &mut lines,
            &range,
            "**Spec Review Status:**",
            spec_review_status,
        )?;
        update_prefixed_line(
            &mut lines,
            &range,
            "**Quality Review Status:**",
            quality_review_status,
        )?;
        update_prefixed_line(&mut lines, &range, "**Assigned Agent:**", assigned_agent)?;

        self.write_lines(lines)
    }

    pub fn mark_top_level_todo(&mut self, task_id: &str, checked: bool) -> Result<()> {
        let mut lines = self.read_lines()?;
        let range =
            find_section_range(&lines, "## TODO List").ok_or_else(|| anyhow!("TODO List section not found"))?;
        let pattern = Regex::new(&format!(
            r"^- \[(?: |x)\] {}\.",
            regex::escape(task_id)
        ))?;
        let index = lines
            .iter()
            .enumerate()
            .find(|(idx, line)| *idx >= range.start && *idx < range.end && pattern.is_match(line))
            .map(|(idx, _)| idx)
            .ok_or_else(|| anyhow!("Top-level todo not found for {task_id}"))?;
        lines[index] = replace_checkbox(&lines[index], checked);
        self.write_lines(lines)
    }

    pub fn mark_step(&mut self, task_id: &str, step_label: &str, checked: bool) -> Result<()> {
        let mut lines = self.read_lines()?;
        let range =
            find_task_range(&lines, task_id).ok_or_else(|| anyhow!("Task block not found: {task_id}"))?;
        let pattern = Regex::new(&format!(
            r"^- \[(?: |x)\] {}:",
            regex::escape(step_label)
        ))?;
        let index = lines
            .iter()
            .enumerate()
            .find(|(idx, line)| *idx >= range.start && *idx < range.end && pattern.is_match(line))
            .map(|(idx, _)| idx)
            .ok_or_else(|| anyhow!("Step not found: {task_id} {step_label}"))?;
        lines[index] = replace_checkbox(&lines[index], checked);
        self.write_lines(lines)
    }

    pub fn all_steps_completed(&mut self, task_id: &str) -> Result<bool> {
        let state = self.read_plan_state()?;
        let task = state
            .tasks
            .into_iter()
            .find(|entry| entry.id == task_id)
            .ok_or_else(|| anyhow!("Task not found: {task_id}"))?;
        Ok(task.steps.iter().all(|step| step.checked))
    }

    pub fn read_final_acceptance(&mut self) -> Result<Vec<FinalAcceptanceItem>> {
        let lines = self.read_lines()?;
        Ok(parse_final_acceptance_items(&lines))
    }

    pub fn all_final_acceptance_checked(&mut self) -> Result<bool> {
        let items = self.read_final_acceptance()?;
        Ok(!items.is_empty() && items.iter().all(|item| item.checked))
    }

    pub fn unchecked_final_acceptance_items(&mut self) -> Result<Vec<String>> {
        Ok(self
            .read_final_acceptance()?
            .into_iter()
            .filter(|item| !item.checked)
            .map(|item| item.text)
            .collect())
    }

    pub fn mark_final_acceptance(&mut self, text: &str, checked: bool) -> Result<()> {
        let mut lines = self.read_lines()?;
        let range = find_section_range(&lines, "## Final Acceptance")
            .ok_or_else(|| anyhow!("Final Acceptance section not found"))?;
        let pattern = Regex::new(&format!(
            r"^- \[(?: |x)\] {}$",
            regex::escape(text)
        ))?;
        let index = lines
            .iter()
            .enumerate()
            .find(|(idx, line)| *idx >= range.start && *idx < range.end && pattern.is_match(line))
            .map(|(idx, _)| idx)
            .ok_or_else(|| anyhow!("Final acceptance item not found: {text}"))?;
        lines[index] = replace_checkbox(&lines[index], checked);
        self.write_lines(lines)
    }

    fn write_lines(&mut self, lines: Vec<String>) -> Result<()> {
        fs::write(
            &self.current_plan_path,
            normalize_document_text(&lines.join("\n")),
        )
        .with_context(|| format!("failed to write plan file: {}", self.current_plan_path))?;
        self.ensure_archived_if_completed()
    }

    fn ensure_archived_if_completed(&mut self) -> Result<()> {
        let archived_path = match completed_plan_path(&self.current_plan_path) {
            Some(path) => path,
            None => return Ok(()),
        };

        if !Path::new(&self.current_plan_path).exists() {
            if Path::new(&archived_path).exists() {
                self.current_plan_path = archived_path;
            }
            return Ok(());
        }

        let text = normalize_line_endings(&fs::read_to_string(&self.current_plan_path)?);
        let lines: Vec<String> = text.split('\n').map(|line| line.to_string()).collect();
        if !is_plan_complete(&lines) {
            return Ok(());
        }

        if let Some(parent) = Path::new(&archived_path).parent() {
            fs::create_dir_all(parent)?;
        }
        fs::rename(&self.current_plan_path, &archived_path)?;
        self.current_plan_path = archived_path.clone();

        let file_name = Path::new(&archived_path)
            .file_name()
            .and_then(|value| value.to_str())
            .ok_or_else(|| anyhow!("archived plan file name missing"))?;
        let rewritten = rewrite_archived_plan_text(&text, file_name);
        if rewritten != text {
            fs::write(&self.current_plan_path, normalize_document_text(&rewritten))?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
struct SectionRange {
    start: usize,
    end: usize,
}

fn parse_execution_status(lines: &[String]) -> Result<PlanExecutionStatus> {
    let range = find_section_range(lines, "## Execution Status")
        .ok_or_else(|| anyhow!("Execution Status section not found"))?;
    Ok(PlanExecutionStatus {
        current_wave: extract_prefixed_line(lines, &range, "- Current wave:")?,
        active_task: extract_prefixed_line(lines, &range, "- Active task:")?,
        blockers: extract_prefixed_line(lines, &range, "- Blockers:")?,
        last_review_result: extract_prefixed_line(lines, &range, "- Last review result:")?,
    })
}

fn parse_tasks(lines: &[String]) -> Result<Vec<PlanTask>> {
    let task_header = Regex::new(r"^### Task ([A-Za-z0-9-]+): (.+)$")?;
    let step_pattern = Regex::new(r"^- \[( |x)\] (Step \d+): (.+)$")?;
    let mut tasks = Vec::new();
    let mut index = 0;

    while index < lines.len() {
        let line = &lines[index];
        let captures = match task_header.captures(line) {
            Some(captures) => captures,
            None => {
                index += 1;
                continue;
            }
        };

        let id = captures.get(1).unwrap().as_str().to_string();
        let title = captures.get(2).unwrap().as_str().to_string();
        let end = find_task_end(lines, index + 1);
        let block = &lines[index..end];
        let steps = block
            .iter()
            .filter_map(|entry| {
                step_pattern.captures(entry).map(|captures| PlanStep {
                    checked: captures.get(1).unwrap().as_str() == "x",
                    label: captures.get(2).unwrap().as_str().to_string(),
                    text: captures.get(3).unwrap().as_str().to_string(),
                })
            })
            .collect::<Vec<_>>();

        tasks.push(PlanTask {
            id: id.clone(),
            title,
            category: extract_field(block, "**Category:**")?,
            owner_role: extract_field(block, "**Owner Role:**")?,
            task_status: extract_field(block, "**Task Status:**")?,
            current_step: extract_field(block, "**Current Step:**")?,
            spec_review_status: normalize_review_status(&extract_field(
                block,
                "**Spec Review Status:**",
            )?),
            quality_review_status: normalize_review_status(&extract_field(
                block,
                "**Quality Review Status:**",
            )?),
            assigned_agent: extract_field(block, "**Assigned Agent:**")?,
            todo_checked: parse_todo_checked(lines, &id)?,
            steps,
        });
        index = end;
    }

    Ok(tasks)
}

fn normalize_review_status(value: &str) -> String {
    match value {
        "pending" | "pass" | "fail" => value.to_string(),
        _ => value.to_string(),
    }
}

fn parse_todo_checked(lines: &[String], task_id: &str) -> Result<bool> {
    let range = match find_section_range(lines, "## TODO List") {
        Some(range) => range,
        None => return Ok(false),
    };
    let pattern = Regex::new(&format!(r"^- \[( |x)\] {}\.", regex::escape(task_id)))?;
    Ok(lines
        .iter()
        .enumerate()
        .find(|(idx, line)| *idx >= range.start && *idx < range.end && pattern.is_match(line))
        .map(|(_, line)| line.starts_with("- [x]"))
        .unwrap_or(false))
}

fn parse_final_acceptance_items(lines: &[String]) -> Vec<FinalAcceptanceItem> {
    let range = match find_section_range(lines, "## Final Acceptance") {
        Some(range) => range,
        None => return Vec::new(),
    };

    lines[(range.start + 1)..range.end]
        .iter()
        .filter_map(|line| {
            let captures = Regex::new(r"^- \[( |x)\] (.+)$")
                .ok()?
                .captures(line)?;
            Some(FinalAcceptanceItem {
                checked: captures.get(1)?.as_str() == "x",
                text: captures.get(2)?.as_str().to_string(),
            })
        })
        .collect()
}

fn update_prefixed_line(
    lines: &mut [String],
    range: &SectionRange,
    prefix: &str,
    next_value: Option<&str>,
) -> Result<()> {
    let Some(next_value) = next_value else {
        return Ok(());
    };
    let index = lines
        .iter()
        .enumerate()
        .find(|(idx, line)| *idx >= range.start && *idx < range.end && line.starts_with(prefix))
        .map(|(idx, _)| idx)
        .ok_or_else(|| anyhow!("Field not found: {prefix}"))?;
    lines[index] = format!("{prefix} {next_value}");
    Ok(())
}

fn extract_prefixed_line(lines: &[String], range: &SectionRange, prefix: &str) -> Result<String> {
    lines.iter()
        .enumerate()
        .find(|(idx, line)| *idx >= range.start && *idx < range.end && line.starts_with(prefix))
        .map(|(_, line)| line[prefix.len()..].trim().to_string())
        .ok_or_else(|| anyhow!("Missing execution status field: {prefix}"))
}

fn extract_field(block: &[String], prefix: &str) -> Result<String> {
    block.iter()
        .find(|entry| entry.starts_with(prefix))
        .map(|line| line[prefix.len()..].trim().to_string())
        .ok_or_else(|| anyhow!("Missing field: {prefix}"))
}

fn find_section_range(lines: &[String], heading: &str) -> Option<SectionRange> {
    let start = lines.iter().position(|line| line.trim() == heading)?;
    let mut end = lines.len();
    for (idx, line) in lines.iter().enumerate().skip(start + 1) {
        if line.starts_with("## ") {
            end = idx;
            break;
        }
    }
    Some(SectionRange { start, end })
}

fn find_task_range(lines: &[String], task_id: &str) -> Option<SectionRange> {
    let pattern = Regex::new(&format!(r"^### Task {}: ", regex::escape(task_id))).ok()?;
    let start = lines.iter().position(|line| pattern.is_match(line))?;
    Some(SectionRange {
        start,
        end: find_task_end(lines, start + 1),
    })
}

fn find_task_end(lines: &[String], from_index: usize) -> usize {
    for (idx, line) in lines.iter().enumerate().skip(from_index) {
        if line.starts_with("### Task ") || line.starts_with("## Final Acceptance") {
            return idx;
        }
    }
    lines.len()
}

fn replace_checkbox(line: &str, checked: bool) -> String {
    let replacement = if checked { "- [x]" } else { "- [ ]" };
    Regex::new(r"^- \[(?: |x)\]")
        .expect("static checkbox regex should compile")
        .replace(line, replacement)
        .to_string()
}

fn resolve_initial_plan_path(plan_path: &str) -> String {
    if let Some(archived_path) = completed_plan_path(plan_path) {
        if !Path::new(plan_path).exists() && Path::new(&archived_path).exists() {
            return archived_path;
        }
    }
    plan_path.to_string()
}

fn completed_plan_path(plan_path: &str) -> Option<String> {
    let normalized = normalize_path_slashes(plan_path);
    let pattern = Regex::new(r"^(.*?)(docs/plans/)active/([^/]+\.md)$").ok()?;
    let captures = pattern.captures(&normalized)?;
    let prefix = captures.get(1)?.as_str();
    let middle = captures.get(2)?.as_str();
    let file_name = captures.get(3)?.as_str();
    Some(to_platform_path(
        &format!("{prefix}{middle}completed/{file_name}"),
        plan_path,
    ))
}

fn normalize_path_slashes(value: &str) -> String {
    value.replace('\\', "/")
}

fn to_platform_path(value: &str, like: &str) -> String {
    if like.contains('\\') {
        value.replace('/', "\\")
    } else {
        value.to_string()
    }
}

fn is_plan_complete(lines: &[String]) -> bool {
    let todo_range = match find_section_range(lines, "## TODO List") {
        Some(range) => range,
        None => return false,
    };
    let todo_pattern = Regex::new(r"^- \[( |x)\] [A-Za-z0-9-]+\.")
        .expect("static todo regex should compile");
    let todo_lines = lines[(todo_range.start + 1)..todo_range.end]
        .iter()
        .filter(|line| todo_pattern.is_match(line))
        .collect::<Vec<_>>();
    if todo_lines.is_empty() || todo_lines.iter().any(|line| line.starts_with("- [ ]")) {
        return false;
    }

    let final_acceptance_items = parse_final_acceptance_items(lines);
    final_acceptance_items.is_empty() || final_acceptance_items.iter().all(|item| item.checked)
}

fn rewrite_archived_plan_text(text: &str, file_name: &str) -> String {
    let active_repo_path = format!("docs/plans/active/{file_name}");
    let completed_repo_path = format!("docs/plans/completed/{file_name}");
    text.replace("Active plan path:", "Completed plan path:")
        .replace(&active_repo_path, &completed_repo_path)
}

fn normalize_document_text(text: &str) -> String {
    format!("{}\n", normalize_line_endings(text).trim_end_matches('\n'))
}

fn normalize_line_endings(text: &str) -> String {
    text.replace("\r\n", "\n")
}
