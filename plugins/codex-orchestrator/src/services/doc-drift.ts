export interface DocDriftResult {
  updateAgents: boolean;
  updateDocsIndex: boolean;
  updateArchitectureDocs: boolean;
  updateProductDocs: boolean;
  reasons: string[];
}

export function checkDocDrift(changedPaths: string[]): DocDriftResult {
  const normalized = changedPaths.map((entry) => entry.replace(/\\/g, "/"));
  const touchesPluginCode = normalized.some((entry) => entry.startsWith("plugins/codex-orchestrator/"));
  const touchesArchitecture = normalized.some((entry) => entry.startsWith("docs/architecture/"));
  const touchesProduct = normalized.some((entry) => entry.startsWith("docs/product/"));
  const touchesRouting = normalized.some((entry) => entry === "AGENTS.md" || entry === "docs/index.md");

  const reasons: string[] = [];
  if (touchesPluginCode) {
    reasons.push("plugin surface changed");
  }
  if (touchesArchitecture) {
    reasons.push("architecture docs already touched");
  }
  if (touchesProduct) {
    reasons.push("product docs already touched");
  }
  if (touchesRouting) {
    reasons.push("routing docs already touched");
  }

  return {
    updateAgents: touchesPluginCode && !normalized.includes("AGENTS.md"),
    updateDocsIndex: touchesPluginCode && !normalized.includes("docs/index.md"),
    updateArchitectureDocs: touchesPluginCode && !touchesArchitecture,
    updateProductDocs: false,
    reasons: reasons.length > 0 ? reasons : ["no drift detected"],
  };
}

