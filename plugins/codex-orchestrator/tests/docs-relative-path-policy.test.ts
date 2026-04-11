import test from "node:test";
import assert from "node:assert/strict";
import { readdirSync, readFileSync, statSync } from "node:fs";
import { dirname, extname, join, resolve, relative } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../../..");

function collectMarkdownFiles(startPath: string): string[] {
  const stat = statSync(startPath);
  if (stat.isFile()) {
    return extname(startPath).toLowerCase() === ".md" ? [startPath] : [];
  }

  const files: string[] = [];
  for (const entry of readdirSync(startPath, { withFileTypes: true })) {
    const fullPath = join(startPath, entry.name);
    if (entry.isDirectory()) {
      files.push(...collectMarkdownFiles(fullPath));
    } else if (entry.isFile() && extname(entry.name).toLowerCase() === ".md") {
      files.push(fullPath);
    }
  }
  return files;
}

function scanForAbsolutePaths(filePath: string): string[] {
  const forbidden = /(\/Users\/|\/home\/|\/mnt\/|\/tmp\/|[A-Za-z]:\\)/;
  return readFileSync(filePath, "utf8")
    .split(/\r?\n/u)
    .flatMap((line, index) => (
      forbidden.test(line)
        ? [`${relative(repoRoot, filePath)}:${index + 1}:${line.trim()}`]
        : []
    ));
}

test("markdown docs do not contain machine-specific absolute filesystem paths", () => {
  const targets = [
    join(repoRoot, "AGENTS.md"),
    join(repoRoot, "README.md"),
    join(repoRoot, "install.md"),
    join(repoRoot, "docs"),
    join(repoRoot, "plugins", "codex-orchestrator", "skills"),
  ];

  const markdownFiles = targets.flatMap((target) => collectMarkdownFiles(target));
  const findings = markdownFiles.flatMap((filePath) => scanForAbsolutePaths(filePath));

  assert.deepEqual(
    findings,
    [],
    `Found machine-specific absolute paths in markdown docs:\n${findings.join("\n")}`,
  );
});
