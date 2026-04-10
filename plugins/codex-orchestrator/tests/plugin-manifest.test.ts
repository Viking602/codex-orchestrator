import test from "node:test";
import assert from "node:assert/strict";
import { existsSync, readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

type PluginManifest = {
  author: {
    name: string;
    email: string;
    url: string;
  };
  homepage: string;
  repository: string;
  interface: {
    websiteURL: string;
    privacyPolicyURL: string;
    termsOfServiceURL: string;
  };
};

const currentDir = dirname(fileURLToPath(import.meta.url));
const pluginRoot = resolve(currentDir, "..");
const repoRoot = resolve(pluginRoot, "..", "..");
const manifestPath = resolve(pluginRoot, ".codex-plugin", "plugin.json");

function readManifest(): PluginManifest {
  return JSON.parse(readFileSync(manifestPath, "utf8")) as PluginManifest;
}

test("plugin manifest uses repository-backed metadata instead of scaffold placeholders", () => {
  const manifest = readManifest();
  const repoUrl = "https://github.com/Viking602/codex-orchestrator";
  const privacyDoc = resolve(repoRoot, "docs/product/privacy-policy.md");
  const termsDoc = resolve(repoRoot, "docs/product/terms-of-service.md");

  assert.ok(existsSync(privacyDoc), "privacy policy doc should exist");
  assert.ok(existsSync(termsDoc), "terms doc should exist");
  assert.equal(manifest.author.name, "Viking");
  assert.equal(manifest.author.email, "chen17090314747@gmail.com");
  assert.equal(manifest.author.url, "https://github.com/Viking602");
  assert.equal(manifest.homepage, repoUrl);
  assert.equal(manifest.repository, repoUrl);
  assert.equal(manifest.interface.websiteURL, repoUrl);
  assert.equal(
    manifest.interface.privacyPolicyURL,
    `${repoUrl}/blob/master/docs/product/privacy-policy.md`,
  );
  assert.equal(
    manifest.interface.termsOfServiceURL,
    `${repoUrl}/blob/master/docs/product/terms-of-service.md`,
  );

  const manifestJson = JSON.stringify(manifest);
  assert.equal(
    manifestJson.includes("example.invalid"),
    false,
    "scaffold placeholder domains must not ship in the manifest",
  );
});
