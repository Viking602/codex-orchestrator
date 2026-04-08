import { mkdirSync } from "node:fs";
import { dirname } from "node:path";
import { DatabaseSync } from "node:sqlite";
import type { PlanStateRecord, ReviewStatus, TaskStateRecord, TaskStatus } from "../types.ts";

export class RuntimeStore {
  private readonly db: DatabaseSync;

  constructor(dbPath: string) {
    mkdirSync(dirname(dbPath), { recursive: true });
    this.db = new DatabaseSync(dbPath);
    this.initialize();
  }

  initialize(): void {
    this.db.exec(`
      CREATE TABLE IF NOT EXISTS plan_state (
        plan_id TEXT PRIMARY KEY,
        plan_path TEXT NOT NULL,
        spec_path TEXT,
        current_wave TEXT,
        active_task_id TEXT,
        last_review_result TEXT,
        updated_at TEXT NOT NULL
      );

      CREATE TABLE IF NOT EXISTS task_state (
        plan_id TEXT NOT NULL,
        task_id TEXT NOT NULL,
        category_id TEXT NOT NULL,
        status TEXT NOT NULL,
        active_step_label TEXT,
        assigned_role TEXT,
        agent_id TEXT,
        write_lease_id TEXT,
        spec_review_status TEXT NOT NULL,
        quality_review_status TEXT NOT NULL,
        retry_count INTEGER NOT NULL DEFAULT 0,
        blocker_type TEXT,
        blocker_message TEXT,
        updated_at TEXT NOT NULL,
        PRIMARY KEY (plan_id, task_id)
      );

      CREATE TABLE IF NOT EXISTS task_run (
        run_id INTEGER PRIMARY KEY AUTOINCREMENT,
        plan_id TEXT NOT NULL,
        task_id TEXT NOT NULL,
        role TEXT NOT NULL,
        agent_id TEXT NOT NULL,
        status TEXT NOT NULL,
        summary TEXT,
        started_at TEXT NOT NULL
      );

      CREATE TABLE IF NOT EXISTS verification_evidence (
        evidence_id INTEGER PRIMARY KEY AUTOINCREMENT,
        plan_id TEXT NOT NULL,
        task_id TEXT NOT NULL,
        kind TEXT NOT NULL,
        command TEXT,
        result TEXT,
        summary TEXT,
        created_at TEXT NOT NULL
      );
    `);
  }

  upsertPlanState(input: {
    planId: string;
    planPath: string;
    specPath?: string | null;
    currentWave?: string | null;
    activeTaskId?: string | null;
    lastReviewResult?: string | null;
  }): PlanStateRecord {
    const now = new Date().toISOString();
    this.db.prepare(`
      INSERT INTO plan_state (plan_id, plan_path, spec_path, current_wave, active_task_id, last_review_result, updated_at)
      VALUES (?, ?, ?, ?, ?, ?, ?)
      ON CONFLICT(plan_id) DO UPDATE SET
        plan_path = excluded.plan_path,
        spec_path = excluded.spec_path,
        current_wave = excluded.current_wave,
        active_task_id = excluded.active_task_id,
        last_review_result = excluded.last_review_result,
        updated_at = excluded.updated_at
    `).run(
      input.planId,
      input.planPath,
      input.specPath ?? null,
      input.currentWave ?? null,
      input.activeTaskId ?? null,
      input.lastReviewResult ?? null,
      now,
    );
    return this.requirePlanState(input.planId);
  }

  upsertTaskState(input: {
    planId: string;
    taskId: string;
    categoryId: string;
    status: TaskStatus;
    activeStepLabel?: string | null;
    assignedRole?: string | null;
    agentId?: string | null;
    writeLeaseId?: string | null;
    specReviewStatus?: ReviewStatus;
    qualityReviewStatus?: ReviewStatus;
    retryCount?: number;
    blockerType?: string | null;
    blockerMessage?: string | null;
  }): TaskStateRecord {
    const now = new Date().toISOString();
    this.db.prepare(`
      INSERT INTO task_state (
        plan_id, task_id, category_id, status, active_step_label, assigned_role, agent_id, write_lease_id,
        spec_review_status, quality_review_status, retry_count, blocker_type, blocker_message, updated_at
      ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
      ON CONFLICT(plan_id, task_id) DO UPDATE SET
        category_id = excluded.category_id,
        status = excluded.status,
        active_step_label = excluded.active_step_label,
        assigned_role = excluded.assigned_role,
        agent_id = excluded.agent_id,
        write_lease_id = excluded.write_lease_id,
        spec_review_status = excluded.spec_review_status,
        quality_review_status = excluded.quality_review_status,
        retry_count = excluded.retry_count,
        blocker_type = excluded.blocker_type,
        blocker_message = excluded.blocker_message,
        updated_at = excluded.updated_at
    `).run(
      input.planId,
      input.taskId,
      input.categoryId,
      input.status,
      input.activeStepLabel ?? null,
      input.assignedRole ?? null,
      input.agentId ?? null,
      input.writeLeaseId ?? null,
      input.specReviewStatus ?? "pending",
      input.qualityReviewStatus ?? "pending",
      input.retryCount ?? 0,
      input.blockerType ?? null,
      input.blockerMessage ?? null,
      now,
    );
    return this.requireTaskState(input.planId, input.taskId);
  }

  recordTaskRun(input: {
    planId: string;
    taskId: string;
    role: string;
    agentId: string;
    status: string;
    summary?: string;
  }): void {
    this.db.prepare(`
      INSERT INTO task_run (plan_id, task_id, role, agent_id, status, summary, started_at)
      VALUES (?, ?, ?, ?, ?, ?, ?)
    `).run(
      input.planId,
      input.taskId,
      input.role,
      input.agentId,
      input.status,
      input.summary ?? null,
      new Date().toISOString(),
    );
  }

  recordEvidence(input: {
    planId: string;
    taskId: string;
    kind: string;
    command?: string;
    result?: string;
    summary?: string;
  }): void {
    this.db.prepare(`
      INSERT INTO verification_evidence (plan_id, task_id, kind, command, result, summary, created_at)
      VALUES (?, ?, ?, ?, ?, ?, ?)
    `).run(
      input.planId,
      input.taskId,
      input.kind,
      input.command ?? null,
      input.result ?? null,
      input.summary ?? null,
      new Date().toISOString(),
    );
  }

  getPlanState(planId: string): PlanStateRecord | undefined {
    const row = this.db.prepare(`
      SELECT
        plan_id AS planId,
        plan_path AS planPath,
        spec_path AS specPath,
        current_wave AS currentWave,
        active_task_id AS activeTaskId,
        last_review_result AS lastReviewResult,
        updated_at AS updatedAt
      FROM plan_state WHERE plan_id = ?
    `).get(planId) as PlanStateRecord | undefined;
    return row;
  }

  getTaskState(planId: string, taskId: string): TaskStateRecord | undefined {
    const row = this.db.prepare(`
      SELECT
             plan_id AS planId,
             task_id AS taskId,
             category_id AS categoryId,
             status,
             active_step_label AS activeStepLabel,
             assigned_role AS assignedRole,
             agent_id AS agentId,
             write_lease_id AS writeLeaseId,
             spec_review_status AS specReviewStatus,
             quality_review_status AS qualityReviewStatus,
             retry_count AS retryCount,
             blocker_type AS blockerType,
             blocker_message AS blockerMessage,
             updated_at AS updatedAt
      FROM task_state WHERE plan_id = ? AND task_id = ?
    `).get(planId, taskId) as TaskStateRecord | undefined;
    return row;
  }

  listStalledTasks(olderThanMs: number): TaskStateRecord[] {
    const threshold = new Date(Date.now() - olderThanMs).toISOString();
    return this.db.prepare(`
      SELECT
             plan_id AS planId,
             task_id AS taskId,
             category_id AS categoryId,
             status,
             active_step_label AS activeStepLabel,
             assigned_role AS assignedRole,
             agent_id AS agentId,
             write_lease_id AS writeLeaseId,
             spec_review_status AS specReviewStatus,
             quality_review_status AS qualityReviewStatus,
             retry_count AS retryCount,
             blocker_type AS blockerType,
             blocker_message AS blockerMessage,
             updated_at AS updatedAt
      FROM task_state
      WHERE updated_at < ? AND status NOT IN ('accepted', 'cancelled', 'blocked')
    `).all(threshold) as TaskStateRecord[];
  }

  private requirePlanState(planId: string): PlanStateRecord {
    const row = this.getPlanState(planId);
    if (!row) throw new Error(`Plan state missing after upsert: ${planId}`);
    return row;
  }

  private requireTaskState(planId: string, taskId: string): TaskStateRecord {
    const row = this.getTaskState(planId, taskId);
    if (!row) throw new Error(`Task state missing after upsert: ${planId}/${taskId}`);
    return row;
  }
}
