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
    assignedRole: "backend-developer",
    agentId: "agent-123",
  });

  const plan = store.getPlanState("plan-1");
  const task = store.getTaskState("plan-1", "T2");

  assert.equal(plan?.activeTaskId, "T2");
  assert.equal(task?.agentId, "agent-123");
  assert.equal(task?.status, "running_impl");
});

test("RuntimeStore acquires and releases write leases", () => {
  const dir = mkdtempSync(join(tmpdir(), "codex-orchestrator-db-"));
  const dbPath = join(dir, "state.db");
  const store = new RuntimeStore(dbPath);

  const lease = store.acquireWriteLease({
    planId: "plan-2",
    taskId: "T9",
    holderAgentId: "agent-lease",
    scope: ["src/**"],
  });
  const activeLease = store.getActiveWriteLease("plan-2", "T9");
  assert.equal(activeLease?.leaseId, lease.leaseId);
  assert.equal(activeLease?.status, "active");

  const released = store.releaseWriteLease(lease.leaseId);
  assert.equal(released.status, "released");
  assert.equal(store.getActiveWriteLease("plan-2", "T9"), undefined);
});
