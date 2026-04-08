import { readFileSync, writeFileSync } from "node:fs";
import type { PlanExecutionStatus, PlanStep, PlanTask, ReviewStatus } from "../types.ts";

function escapeRegex(value: string): string {
  return value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

function normalizeReviewStatus(value: string): ReviewStatus | string {
  if (value === "pending" || value === "pass" || value === "fail") {
    return value;
  }
  return value;
}

export class PlanDocument {
  readonly planPath: string;

  constructor(planPath: string) {
    this.planPath = planPath;
  }

  readText(): string {
    return readFileSync(this.planPath, "utf8");
  }

  readLines(): string[] {
    return this.readText().split("\n");
  }

  readPlanState(): { executionStatus: PlanExecutionStatus; tasks: PlanTask[] } {
    const lines = this.readLines();
    return {
      executionStatus: this.parseExecutionStatus(lines),
      tasks: this.parseTasks(lines),
    };
  }

  updateExecutionStatus(update: Partial<PlanExecutionStatus>): void {
    const lines = this.readLines();
    const range = this.findSectionRange(lines, "## Execution Status");
    if (!range) throw new Error("Execution Status section not found");

    const fieldMap: Array<[keyof PlanExecutionStatus, string]> = [
      ["currentWave", "- Current wave:"],
      ["activeTask", "- Active task:"],
      ["blockers", "- Blockers:"],
      ["lastReviewResult", "- Last review result:"],
    ];

    for (const [field, prefix] of fieldMap) {
      const nextValue = update[field];
      if (nextValue === undefined) continue;
      const index = lines.findIndex((line, idx) => idx >= range.start && idx < range.end && line.startsWith(prefix));
      if (index === -1) throw new Error(`Execution status field not found: ${prefix}`);
      lines[index] = `${prefix} ${nextValue}`;
    }

    this.writeLines(lines);
  }

  updateTaskMetadata(taskId: string, update: {
    taskStatus?: string;
    currentStep?: string;
    specReviewStatus?: string;
    qualityReviewStatus?: string;
    assignedAgent?: string;
  }): void {
    const lines = this.readLines();
    const range = this.findTaskRange(lines, taskId);
    if (!range) throw new Error(`Task block not found: ${taskId}`);

    const fieldMap: Array<[string, string | undefined]> = [
      ["**Task Status:**", update.taskStatus],
      ["**Current Step:**", update.currentStep],
      ["**Spec Review Status:**", update.specReviewStatus],
      ["**Quality Review Status:**", update.qualityReviewStatus],
      ["**Assigned Agent:**", update.assignedAgent],
    ];

    for (const [prefix, nextValue] of fieldMap) {
      if (nextValue === undefined) continue;
      const index = lines.findIndex((line, idx) => idx >= range.start && idx < range.end && line.startsWith(prefix));
      if (index === -1) throw new Error(`Task metadata field not found: ${prefix}`);
      lines[index] = `${prefix} ${nextValue}`;
    }

    this.writeLines(lines);
  }

  markTopLevelTodo(taskId: string, checked: boolean): void {
    const lines = this.readLines();
    const range = this.findSectionRange(lines, "## TODO List");
    if (!range) throw new Error("TODO List section not found");
    const regex = new RegExp(`^- \\[(?: |x)\\] ${escapeRegex(taskId)}\\.`);
    const index = lines.findIndex((line, idx) => idx >= range.start && idx < range.end && regex.test(line));
    if (index === -1) throw new Error(`Top-level todo not found for ${taskId}`);
    lines[index] = lines[index].replace(/^- \[(?: |x)\]/, checked ? "- [x]" : "- [ ]");
    this.writeLines(lines);
  }

  markStep(taskId: string, stepLabel: string, checked: boolean): void {
    const lines = this.readLines();
    const range = this.findTaskRange(lines, taskId);
    if (!range) throw new Error(`Task block not found: ${taskId}`);
    const regex = new RegExp(`^- \\[(?: |x)\\] ${escapeRegex(stepLabel)}:`);
    const index = lines.findIndex((line, idx) => idx >= range.start && idx < range.end && regex.test(line));
    if (index === -1) throw new Error(`Step not found: ${taskId} ${stepLabel}`);
    lines[index] = lines[index].replace(/^- \[(?: |x)\]/, checked ? "- [x]" : "- [ ]");
    this.writeLines(lines);
  }

  allStepsCompleted(taskId: string): boolean {
    const task = this.readPlanState().tasks.find((entry) => entry.id === taskId);
    if (!task) throw new Error(`Task not found: ${taskId}`);
    return task.steps.every((step) => step.checked);
  }

  private parseExecutionStatus(lines: string[]): PlanExecutionStatus {
    const range = this.findSectionRange(lines, "## Execution Status");
    if (!range) throw new Error("Execution Status section not found");
    const extract = (prefix: string): string => {
      const line = lines.find((entry, idx) => idx >= range.start && idx < range.end && entry.startsWith(prefix));
      if (!line) throw new Error(`Missing execution status field: ${prefix}`);
      return line.slice(prefix.length).trim();
    };
    return {
      currentWave: extract("- Current wave:"),
      activeTask: extract("- Active task:"),
      blockers: extract("- Blockers:"),
      lastReviewResult: extract("- Last review result:"),
    };
  }

  private parseTasks(lines: string[]): PlanTask[] {
    const tasks: PlanTask[] = [];
    for (let index = 0; index < lines.length; index += 1) {
      const line = lines[index];
      const match = line.match(/^### Task (T\d+): (.+)$/);
      if (!match) continue;
      const [, id, title] = match;
      const end = this.findTaskEnd(lines, index + 1);
      const block = lines.slice(index, end);
      tasks.push({
        id,
        title,
        taskStatus: this.extractField(block, "**Task Status:**"),
        currentStep: this.extractField(block, "**Current Step:**"),
        specReviewStatus: normalizeReviewStatus(this.extractField(block, "**Spec Review Status:**")),
        qualityReviewStatus: normalizeReviewStatus(this.extractField(block, "**Quality Review Status:**")),
        assignedAgent: this.extractField(block, "**Assigned Agent:**"),
        todoChecked: this.parseTodoChecked(lines, id),
        steps: this.parseSteps(block),
      });
      index = end - 1;
    }
    return tasks;
  }

  private parseSteps(block: string[]): PlanStep[] {
    const steps: PlanStep[] = [];
    for (const line of block) {
      const match = line.match(/^- \[( |x)\] (Step \d+): (.+)$/);
      if (!match) continue;
      steps.push({
        label: match[2],
        text: match[3],
        checked: match[1] === "x",
      });
    }
    return steps;
  }

  private parseTodoChecked(lines: string[], taskId: string): boolean {
    const range = this.findSectionRange(lines, "## TODO List");
    if (!range) return false;
    const regex = new RegExp(`^- \\[( |x)\\] ${escapeRegex(taskId)}\\.`);
    const line = lines.find((entry, idx) => idx >= range.start && idx < range.end && regex.test(entry));
    return line?.startsWith("- [x]") ?? false;
  }

  private extractField(block: string[], prefix: string): string {
    const line = block.find((entry) => entry.startsWith(prefix));
    if (!line) throw new Error(`Missing field: ${prefix}`);
    return line.slice(prefix.length).trim();
  }

  private findSectionRange(lines: string[], heading: string): { start: number; end: number } | null {
    const startIndex = lines.findIndex((line) => line.trim() === heading);
    if (startIndex === -1) return null;
    let endIndex = lines.length;
    for (let index = startIndex + 1; index < lines.length; index += 1) {
      if (lines[index].startsWith("## ")) {
        endIndex = index;
        break;
      }
    }
    return { start: startIndex, end: endIndex };
  }

  private findTaskRange(lines: string[], taskId: string): { start: number; end: number } | null {
    const regex = new RegExp(`^### Task ${escapeRegex(taskId)}: `);
    const startIndex = lines.findIndex((line) => regex.test(line));
    if (startIndex === -1) return null;
    return { start: startIndex, end: this.findTaskEnd(lines, startIndex + 1) };
  }

  private findTaskEnd(lines: string[], fromIndex: number): number {
    for (let index = fromIndex; index < lines.length; index += 1) {
      if (lines[index].startsWith("### Task ") || lines[index].startsWith("## Final Acceptance")) {
        return index;
      }
    }
    return lines.length;
  }

  private writeLines(lines: string[]): void {
    writeFileSync(this.planPath, `${lines.join("\n").replace(/\n+$/u, "")}\n`, "utf8");
  }
}
