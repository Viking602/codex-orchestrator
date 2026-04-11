import test from "node:test";
import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import {
  existsSync,
  mkdirSync,
  mkdtempSync,
  readdirSync,
  readFileSync,
  rmSync,
  writeFileSync,
} from "node:fs";
import { dirname, join, resolve } from "node:path";
import { tmpdir } from "node:os";
import { fileURLToPath } from "node:url";

const currentDir = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(currentDir, "..");
const scriptPath = join(repoRoot, "scripts", "install-codex-orchestrator.sh");
const repoMarketplacePath = join(repoRoot, ".agents", "plugins", "marketplace.json");
const normalizedScriptPath = join(repoRoot, "scripts", ".install-codex-orchestrator.test.sh");

function toWslPath(path: string): string {
  return path.replace(/^([A-Za-z]):/u, (_, drive: string) => `/mnt/${drive.toLowerCase()}`)
    .replace(/\\/gu, "/");
}

function shellQuote(value: string): string {
  return `'${value.replace(/'/gu, `'\\''`)}'`;
}

function listFilesRecursive(root: string): string[] {
  const entries = readdirSync(root, { withFileTypes: true });
  return entries.flatMap((entry) => {
    const fullPath = join(root, entry.name);
    return entry.isDirectory() ? listFilesRecursive(fullPath) : [fullPath];
  });
}

function runInstaller(args: string[]): string {
  const normalizedScript = readFileSync(scriptPath, "utf8").replace(/\r\n/gu, "\n");
  writeFileSync(normalizedScriptPath, normalizedScript, "utf8");

  const runtimeArgs = args.map((arg) => (/^[A-Za-z]:\\/u.test(arg) ? toWslPath(arg) : arg));
  const quotedArgs = runtimeArgs.map(shellQuote).join(" ");
  const command = [
    `cd ${shellQuote(toWslPath(repoRoot))}`,
    `bash ${shellQuote(toWslPath(normalizedScriptPath))} ${quotedArgs}`,
  ].join(" && ");
  try {
    return execFileSync("bash", ["-lc", command], {
      encoding: "utf8",
      env: { ...process.env },
    });
  } finally {
    rmSync(normalizedScriptPath, { force: true });
  }
}

test("repo marketplace advertises codex-orchestrator from the repository plugin path", () => {
  assert.equal(existsSync(repoMarketplacePath), true);

  const marketplaceJson = JSON.parse(readFileSync(repoMarketplacePath, "utf8")) as {
    name: string;
    interface?: { displayName?: string };
    plugins: Array<{
      name?: string;
      source?: { source?: string; path?: string };
      policy?: { installation?: string; authentication?: string };
      category?: string;
    }>;
  };

  assert.equal(marketplaceJson.name, "local-plugins");
  assert.equal(marketplaceJson.interface?.displayName, "Local Plugins");

  const pluginEntry = marketplaceJson.plugins.find((plugin) => plugin.name === "codex-orchestrator");
  assert.ok(pluginEntry);
  assert.equal(pluginEntry.source?.source, "local");
  assert.equal(pluginEntry.source?.path, "./plugins/codex-orchestrator");
  assert.equal(pluginEntry.policy?.installation, "AVAILABLE");
  assert.equal(pluginEntry.policy?.authentication, "ON_INSTALL");
  assert.equal(pluginEntry.category, "Coding");
});

test("installer dry-run performs no writes", () => {
  const root = mkdtempSync(join(tmpdir(), "codex-orchestrator-install-"));
  const pluginHome = join(root, ".codex/plugins");
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
  assert.equal(existsSync(join(pluginHome, "cache")), false);
  assert.equal(existsSync(marketplace), false);
  assert.equal(existsSync(agentDir), false);
  assert.equal(existsSync(configPath), false);
});

test("installer copy mode bootstraps personal marketplace, cache install, and enabled state", () => {
  const root = mkdtempSync(join(tmpdir(), "codex-orchestrator-install-"));
  const pluginHome = join(root, ".codex/plugins");
  const marketplace = join(root, ".agents/plugins/marketplace.json");
  const agentDir = join(root, ".codex/agents");
  const configPath = join(root, ".codex/config.toml");
  const installedPath = join(
    pluginHome,
    "cache/local-plugins/codex-orchestrator/local",
  );

  runInstaller([
    "--copy",
    "--plugin-home", pluginHome,
    "--marketplace-path", marketplace,
    "--agent-dir", agentDir,
    "--config-path", configPath,
  ]);

  assert.equal(existsSync(join(pluginHome, "codex-orchestrator/.codex-plugin/plugin.json")), true);
  assert.equal(existsSync(join(installedPath, ".codex-plugin/plugin.json")), true);
  assert.equal(existsSync(marketplace), true);
  assert.equal(existsSync(join(agentDir, "backend-developer.toml")), true);
  assert.equal(existsSync(configPath), true);

  const marketplaceJson = JSON.parse(readFileSync(marketplace, "utf8")) as {
    plugins: Array<{
      name?: string;
      source?: { path?: string };
      policy?: { installation?: string; authentication?: string };
    }>;
  };
  const pluginEntry = marketplaceJson.plugins.find((plugin) => plugin.name === "codex-orchestrator");
  assert.ok(pluginEntry);
  assert.equal(pluginEntry.source?.path, "./.codex/plugins/codex-orchestrator");
  assert.equal(pluginEntry.policy?.installation, "AVAILABLE");
  assert.equal(pluginEntry.policy?.authentication, "ON_INSTALL");

  const configText = readFileSync(configPath, "utf8");
  assert.match(configText, /\[plugins\."codex-orchestrator@local-plugins"\]/u);
  assert.match(configText, /enabled = true/u);
  assert.doesNotMatch(configText, /apps = true/u);
});

test("installer preserves unrelated feature flags and backs up conflicting agents", () => {
  const root = mkdtempSync(join(tmpdir(), "codex-orchestrator-install-"));
  const pluginHome = join(root, ".codex/plugins");
  const marketplace = join(root, ".agents/plugins/marketplace.json");
  const agentDir = join(root, ".codex/agents");
  const configPath = join(root, ".codex/config.toml");

  mkdirSync(agentDir, { recursive: true });
  mkdirSync(dirname(configPath), { recursive: true });
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
  const backupTree = listFilesRecursive(backupRoot).join("\n");
  assert.match(backupTree, /backend-developer\.toml/u);

  const configText = readFileSync(configPath, "utf8");
  assert.match(configText, /\[features\]\napps = false/u);
  assert.match(configText, /\[plugins\."codex-orchestrator@local-plugins"\]\nenabled = true/u);
  assert.doesNotMatch(configText, /apps = true/u);
});
