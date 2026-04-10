import test from "node:test";
import assert from "node:assert/strict";
import { mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { CategoryRegistry } from "../src/services/category-registry.ts";

test("CategoryRegistry resolves explicit category", () => {
  const dir = mkdtempSync(join(tmpdir(), "codex-orchestrator-category-"));
  const file = join(dir, "categories.toml");
  writeFileSync(file, `
[plan]
intent = "planning"
preferred_role = "harness-planner"
allowed_roles = ["harness-planner"]
write_policy = "docs-only"
requires_plan = false
requires_spec_review = true
requires_quality_review = true
parallelism = "single"
reuse_policy = "same_task_only"
completion_contract = ["artifact_written"]
`.trim());
  const registry = CategoryRegistry.fromToml(file);
  const resolved = registry.resolve({
    title: "anything",
    description: "anything",
    explicitCategory: "plan",
  });
  assert.equal(resolved.categoryId, "plan");
  assert.equal(resolved.reason, "explicit_category");
});

test("CategoryRegistry resolves research by keywords", () => {
  const dir = mkdtempSync(join(tmpdir(), "codex-orchestrator-category-"));
  const file = join(dir, "categories.toml");
  writeFileSync(file, `
[research]
intent = "research"
preferred_role = "search-specialist"
allowed_roles = ["search-specialist"]
write_policy = "read-only"
requires_plan = false
requires_spec_review = false
requires_quality_review = false
parallelism = "parallel"
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
reuse_policy = "same_task_same_role_same_scope"
completion_contract = ["task_accepted"]
`.trim());
  const registry = CategoryRegistry.fromToml(file);
  const resolved = registry.resolve({
    title: "Analyze codebase",
    description: "Investigate repository patterns",
  });
  assert.equal(resolved.categoryId, "research");
});
