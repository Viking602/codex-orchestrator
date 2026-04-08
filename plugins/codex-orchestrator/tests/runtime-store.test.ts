import test from "node:test";
import assert from "node:assert/strict";
import { mkdtempSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { RuntimeStore } from "../src/services/runtime-store.ts";

test("RuntimeStore persists plan and task state", () => {
  const dir = mkdtempSync(join(tmpdir(), "codex-orchestrator-db-"));
  const dbPath = join(dir, "state.db");
  const store = new RuntimeStore(dbPath);

  store.upsertPlanState({
    planId: "plan-1",
    planPath: "/tmp/plan.md",
    currentWave: "Wave 2",
    activeTaskId: "T2",
  });

  store.upsertTaskState({
    planId: "plan-1",
    taskId: "T2",
    categoryId: "backend-impl",
    status: "running_impl",
    assignedRole: "mcp-developer",
    agentId: "agent-123",
  });

  const plan = store.getPlanState("plan-1");
  const task = store.getTaskState("plan-1", "T2");

  assert.equal(plan?.activeTaskId, "T2");
  assert.equal(task?.agentId, "agent-123");
  assert.equal(task?.status, "running_impl");
});
