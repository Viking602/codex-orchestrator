import test from "node:test";
import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { tmpdir } from "node:os";
import { mkdtempSync } from "node:fs";

const repoRoot = "/Users/viking/agents_dev/project03";
const scriptPath = join(repoRoot, "scripts/install-codex-orchestrator.sh");

function runInstaller(args: string[], cwd = repoRoot): string {
  return execFileSync("bash", [scriptPath, ...args], {
    cwd,
    encoding: "utf8",
    env: { ...process.env },
  });
}

test("installer dry-run performs no writes", () => {
  const root = mkdtempSync(join(tmpdir(), "codex-orchestrator-install-"));
  const pluginHome = join(root, "plugins");
  const marketplace = join(root, ".agents/plugins/marketplace.json");
  const agentDir = join(root, ".codex/agents");
  const configPath = join(root, ".codex/config.toml");

  const output = runInstaller([
    "--dry-run",
    "--plugin-home", pluginHome,
    "--marketplace-path", marketplace,
    "--agent-dir", agentDir,
    "--config-path", configPath,
  ]);

  assert.match(output, /\[dry-run\]/u);
  assert.equal(existsSync(join(pluginHome, "codex-orchestrator")), false);
  assert.equal(existsSync(marketplace), false);
  assert.equal(existsSync(agentDir), false);
  assert.equal(existsSync(configPath), false);
});

test("installer copy mode writes plugin, marketplace, and agent files", () => {
  const root = mkdtempSync(join(tmpdir(), "codex-orchestrator-install-"));
  const pluginHome = join(root, "plugins");
  const marketplace = join(root, ".agents/plugins/marketplace.json");
  const agentDir = join(root, ".codex/agents");
  const configPath = join(root, ".codex/config.toml");

  runInstaller([
    "--copy",
    "--plugin-home", pluginHome,
    "--marketplace-path", marketplace,
    "--agent-dir", agentDir,
    "--config-path", configPath,
  ]);

  assert.equal(existsSync(join(pluginHome, "codex-orchestrator/.codex-plugin/plugin.json")), true);
  assert.equal(existsSync(marketplace), true);
  assert.equal(existsSync(join(agentDir, "backend-developer.toml")), true);
  assert.equal(existsSync(configPath), true);

  const marketplaceJson = JSON.parse(readFileSync(marketplace, "utf8"));
  const pluginEntry = marketplaceJson.plugins.find((plugin: { name?: string }) => plugin.name === "codex-orchestrator");
  assert.ok(pluginEntry);
  assert.equal(pluginEntry.source.path, "./plugins/codex-orchestrator");

  const configText = readFileSync(configPath, "utf8");
  assert.match(configText, /\[plugins\."codex-orchestrator@local-plugins"\]/u);
  assert.match(configText, /enabled = true/u);
  assert.match(configText, /apps = true/u);
});

test("installer backs up conflicting agent files before replacement", () => {
  const root = mkdtempSync(join(tmpdir(), "codex-orchestrator-install-"));
  const pluginHome = join(root, "plugins");
  const marketplace = join(root, ".agents/plugins/marketplace.json");
  const agentDir = join(root, ".codex/agents");
  const configPath = join(root, ".codex/config.toml");

  mkdirSync(agentDir, { recursive: true });
  writeFileSync(join(agentDir, "backend-developer.toml"), "old agent\n", "utf8");
  writeFileSync(
    configPath,
    "[features]\napps = false\n\n[plugins.\"codex-orchestrator@local-plugins\"]\nenabled = false\n",
    "utf8",
  );

  runInstaller([
    "--copy",
    "--plugin-home", pluginHome,
    "--marketplace-path", marketplace,
    "--agent-dir", agentDir,
    "--config-path", configPath,
  ]);

  const backupRoot = join(agentDir, ".codex-orchestrator-backups");
  assert.equal(existsSync(backupRoot), true);
  const backupTree = execFileSync("find", [backupRoot, "-type", "f"], { encoding: "utf8" });
  assert.match(backupTree, /backend-developer\.toml/u);

  const configText = readFileSync(configPath, "utf8");
  assert.match(configText, /\[plugins\."codex-orchestrator@local-plugins"\]/u);
  assert.match(configText, /enabled = true/u);
  assert.match(configText, /apps = true/u);
});
