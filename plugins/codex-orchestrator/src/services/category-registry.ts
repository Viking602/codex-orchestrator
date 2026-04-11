import { readFileSync } from "node:fs";
import type { CategoryDefinition, CategoryResolution } from "../types.ts";
import type { DelegationPreference } from "../types.ts";

type RawCategory = {
  intent?: string;
  preferred_role?: string;
  allowed_roles?: string[];
  write_policy?: string;
  requires_plan?: boolean;
  requires_spec_review?: boolean;
  requires_quality_review?: boolean;
  parallelism?: string;
  delegation_preference?: string;
  reuse_policy?: string;
  completion_contract?: string[];
};

type RawCategoryRecord = Record<string, RawCategory>;

function parseSimpleToml(source: string): RawCategoryRecord {
  const result: RawCategoryRecord = {};
  let currentSection: string | null = null;

  for (const rawLine of source.split(/\r?\n/u)) {
    const line = rawLine.trim();
    if (!line || line.startsWith("#")) continue;

    const sectionMatch = line.match(/^\[([^\]]+)\]$/u);
    if (sectionMatch) {
      currentSection = sectionMatch[1].trim();
      result[currentSection] = {};
      continue;
    }

    if (!currentSection) {
      throw new Error(`TOML key outside section: ${line}`);
    }

    const [rawKey, rawValue] = line.split("=", 2);
    if (!rawKey || rawValue === undefined) {
      throw new Error(`Invalid TOML line: ${line}`);
    }

    const key = rawKey.trim();
    const value = rawValue.trim();
    result[currentSection][key as keyof RawCategory] = parseTomlValue(value) as never;
  }

  return result;
}

function parseTomlValue(value: string): string | boolean | string[] {
  if (value === "true") return true;
  if (value === "false") return false;
  if (value.startsWith("[") && value.endsWith("]")) {
    const content = value.slice(1, -1).trim();
    if (!content) return [];
    return content
      .split(",")
      .map((entry) => entry.trim())
      .map((entry) => entry.replace(/^"(.*)"$/u, "$1"));
  }
  return value.replace(/^"(.*)"$/u, "$1");
}

export class CategoryRegistry {
  private readonly categories: Map<string, CategoryDefinition>;

  constructor(definitions: CategoryDefinition[]) {
    this.categories = new Map(definitions.map((definition) => [definition.id, definition]));
  }

  static fromToml(filePath: string): CategoryRegistry {
    const source = readFileSync(filePath, "utf8");
    const parsed = parseSimpleToml(source);
    const definitions = Object.entries(parsed).map(([id, raw]) => ({
      id,
      intent: raw.intent ?? id,
      preferredRole: raw.preferred_role ?? "default",
      allowedRoles: raw.allowed_roles ?? [raw.preferred_role ?? "default"],
      writePolicy: raw.write_policy ?? "read-only",
      requiresPlan: raw.requires_plan ?? false,
      requiresSpecReview: raw.requires_spec_review ?? false,
      requiresQualityReview: raw.requires_quality_review ?? false,
      parallelism: raw.parallelism ?? "single",
      delegationPreference: normalizeDelegationPreference(id, raw.delegation_preference),
      reusePolicy: raw.reuse_policy ?? "no_reuse",
      completionContract: raw.completion_contract ?? [],
    }));
    return new CategoryRegistry(definitions);
  }

  get(categoryId: string): CategoryDefinition | undefined {
    return this.categories.get(categoryId);
  }

  list(): CategoryDefinition[] {
    return Array.from(this.categories.values());
  }

  resolve(input: {
    title: string;
    description: string;
    explicitCategory?: string;
  }): CategoryResolution {
    if (input.explicitCategory) {
      const explicit = this.get(input.explicitCategory);
      if (!explicit) {
        throw new Error(`Unknown category: ${input.explicitCategory}`);
      }
      return {
        categoryId: explicit.id,
        reason: "explicit_category",
        category: explicit,
      };
    }

    const haystack = `${input.title}\n${input.description}`.toLowerCase();

    const planners = ["plan", "spec", "design", "architecture", "contract", "decompose"];
    if (planners.some((term) => haystack.includes(term))) {
      const category = this.require("plan");
      return { categoryId: category.id, reason: "planning_keywords", category };
    }

    const reviewers = ["review", "verify", "evaluation", "evaluator", "qa", "quality"];
    if (reviewers.some((term) => haystack.includes(term))) {
      const category = this.require("review");
      return { categoryId: category.id, reason: "review_keywords", category };
    }

    const researchers = ["research", "analyze", "analysis", "inspect", "scan", "find", "investigate", "map"];
    if (researchers.some((term) => haystack.includes(term))) {
      const category = this.require("research");
      return { categoryId: category.id, reason: "research_keywords", category };
    }

    const category = this.require("backend-impl");
    return { categoryId: category.id, reason: "default_backend_impl", category };
  }

  private require(categoryId: string): CategoryDefinition {
    const definition = this.get(categoryId);
    if (!definition) {
      throw new Error(`Missing required category definition: ${categoryId}`);
    }
    return definition;
  }
}

function normalizeDelegationPreference(categoryId: string, value: string | undefined): DelegationPreference {
  if (value === undefined) {
    throw new Error(`Category ${categoryId} is missing required delegation_preference`);
  }
  if (value === "parent-only" || value === "prefer-subagent" || value === "subagent-required") {
    return value;
  }
  throw new Error(`Category ${categoryId} has invalid delegation_preference: ${value}`);
}
