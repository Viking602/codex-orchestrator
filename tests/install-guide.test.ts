import test from "node:test";
import assert from "node:assert/strict";
import { existsSync, readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const currentDir = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(currentDir, "..");
const legacyInstallerPath = join(repoRoot, "scripts", "install-codex-orchestrator.sh");
const repoMarketplacePath = join(repoRoot, ".agents", "plugins", "marketplace.json");
const installGuidePath = join(repoRoot, "install.md");
const readmePath = join(repoRoot, "README.md");
const docsIndexPath = join(repoRoot, "docs", "index.md");
const agentsPath = join(repoRoot, "AGENTS.md");

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

test("install guide documents direct Codex-driven install and update flow", () => {
  assert.equal(existsSync(installGuidePath), true);

  const installGuide = readFileSync(installGuidePath, "utf8");
  assert.match(installGuide, /install and update/i);
  assert.match(installGuide, /do the work yourself/i);
  assert.match(installGuide, /~\/\.codex\/plugins\/codex-orchestrator/u);
  assert.match(installGuide, /~\/\.codex\/plugins\/cache\/local-plugins\/codex-orchestrator\/local/u);
  assert.match(installGuide, /~\/\.agents\/plugins\/marketplace\.json/u);
  assert.match(installGuide, /~\/\.codex\/config\.toml/u);
  assert.match(installGuide, /AGENTS\.override\.md/u);
  assert.match(installGuide, /codex exec/u);
  assert.doesNotMatch(installGuide, /install-codex-orchestrator\.sh/u);
  assert.doesNotMatch(installGuide, /run the installer/i);
});

test("README routes installation through install.md instead of a shell installer", () => {
  const readme = readFileSync(readmePath, "utf8");

  assert.match(readme, /install\.md/u);
  assert.match(readme, /Codex-guided install/i);
  assert.doesNotMatch(readme, /install-codex-orchestrator\.sh/u);
  assert.doesNotMatch(readme, /Run the installer from the repository root/i);
});

test("current routing docs do not present the shell installer as the active install flow", () => {
  const docsIndex = readFileSync(docsIndexPath, "utf8");
  const agents = readFileSync(agentsPath, "utf8");

  assert.match(docsIndex, /Root install guide/u);
  assert.match(agents, /Read \[install\.md\]/u);
  assert.doesNotMatch(docsIndex, /install-codex-orchestrator\.sh/u);
  assert.doesNotMatch(agents, /install-codex-orchestrator\.sh/u);
});

test("legacy shell installer file is removed from the active repository surface", () => {
  assert.equal(existsSync(legacyInstallerPath), false);
});
