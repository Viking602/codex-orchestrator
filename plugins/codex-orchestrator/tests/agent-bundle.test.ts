import test from "node:test";
import assert from "node:assert/strict";
import { existsSync, readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { CategoryRegistry } from "../src/services/category-registry.ts";

const currentDir = dirname(fileURLToPath(import.meta.url));
const pluginRoot = resolve(currentDir, "..");
const bundleDir = resolve(pluginRoot, "codex/agents");
const openAiManifestPath = resolve(pluginRoot, "agents/openai.yaml");
const categoriesPath = resolve(pluginRoot, "config/categories.toml");

const expectedBundle = [
  "harness-planner",
  "harness-dispatch-gate",
  "search-specialist",
  "backend-developer",
  "harness-evaluator",
  "harness-doc-gardener",
];

test("plugin bundle exposes the expected codex agent inventory", () => {
  const manifest = readFileSync(openAiManifestPath, "utf8");

  assert.match(manifest, /path:\s+"..\/codex\/agents\/"/u);
  assert.match(manifest, /format:\s+"toml"/u);

  for (const agent of expectedBundle) {
    assert.equal(
      existsSync(resolve(bundleDir, `${agent}.toml`)),
      true,
      `missing bundled agent file for ${agent}`,
    );
    assert.match(manifest, new RegExp(`- ${agent}\\b`, "u"));
  }
});

test("category preferred roles stay aligned with the bundled happy-path agents", () => {
  const registry = CategoryRegistry.fromToml(categoriesPath);
  const preferredRoles = [
    registry.get("plan")?.preferredRole,
    registry.get("research")?.preferredRole,
    registry.get("backend-impl")?.preferredRole,
    registry.get("review")?.preferredRole,
  ];

  for (const role of preferredRoles) {
    assert.ok(role, "preferred role should be defined");
    assert.equal(
      expectedBundle.includes(role),
      true,
      `preferred role ${role} must be present in the bundled agent set`,
    );
  }
});

test("bundled agents keep the GPT-optimized contract anchors", () => {
  const backend = readFileSync(resolve(bundleDir, "backend-developer.toml"), "utf8");
  const planner = readFileSync(resolve(bundleDir, "harness-planner.toml"), "utf8");
  const evaluator = readFileSync(resolve(bundleDir, "harness-evaluator.toml"), "utf8");
  const search = readFileSync(resolve(bundleDir, "search-specialist.toml"), "utf8");

  assert.match(backend, /<intent_gate>/u);
  assert.match(backend, /<verification_loop>/u);
  assert.match(backend, /<completion_contract>/u);

  assert.match(planner, /<intent_gate>/u);
  assert.match(planner, /<planning_contract>/u);

  assert.match(evaluator, /findings-first/u);
  assert.match(evaluator, /<completion_contract>/u);

  assert.match(search, /<search_rules>/u);
  assert.match(search, /<return_contract>/u);
});
