use std::{fs, path::Path};

use anyhow::{anyhow, Context, Result};
use chrono::Utc;
use rusqlite::{params, Connection, OptionalExtension};
use serde::Serialize;
use uuid::Uuid;

use crate::types::{PlanStateRecord, TaskStateRecord, WriteLeaseRecord};

#[derive(Debug, Clone)]
pub struct TaskStateUpsertInput {
    pub plan_id: String,
    pub task_id: String,
    pub category_id: String,
    pub status: String,
    pub active_step_label: Option<String>,
    pub assigned_role: Option<String>,
    pub agent_id: Option<String>,
    pub write_lease_id: Option<String>,
    pub spec_review_status: String,
    pub quality_review_status: String,
    pub retry_count: i64,
    pub blocker_type: Option<String>,
    pub blocker_message: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PlanStateUpsertInput {
    pub plan_id: String,
    pub plan_path: String,
    pub spec_path: Option<String>,
    pub current_wave: Option<String>,
    pub active_task_id: Option<String>,
    pub last_review_result: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct EvidenceRecord {
    pub kind: String,
    pub command: Option<String>,
    pub result: Option<String>,
    pub summary: Option<String>,
    pub created_at: String,
}

#[derive(Debug)]
pub struct RuntimeStore {
    conn: Connection,
}

impl RuntimeStore {
    pub fn new(db_path: &str) -> Result<Self> {
        if let Some(parent) = Path::new(db_path).parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("failed to create runtime-store directory for {db_path}"))?;
        }
        let conn = Connection::open(db_path)
            .with_context(|| format!("failed to open runtime-store DB: {db_path}"))?;
        let store = Self { conn };
        store.initialize()?;
        Ok(store)
    }

    fn initialize(&self) -> Result<()> {
        self.conn.execute_batch(
            r#"
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

      CREATE TABLE IF NOT EXISTS write_lease (
        lease_id TEXT PRIMARY KEY,
        plan_id TEXT NOT NULL,
        task_id TEXT NOT NULL,
        holder_agent_id TEXT NOT NULL,
        scope_json TEXT NOT NULL,
        status TEXT NOT NULL,
        created_at TEXT NOT NULL,
        released_at TEXT
      );
        "#,
        )?;
        Ok(())
    }

    pub fn upsert_plan_state(&self, input: PlanStateUpsertInput) -> Result<PlanStateRecord> {
        let now = now_iso();
        self.conn.execute(
            r#"
      INSERT INTO plan_state (plan_id, plan_path, spec_path, current_wave, active_task_id, last_review_result, updated_at)
      VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
      ON CONFLICT(plan_id) DO UPDATE SET
        plan_path = excluded.plan_path,
        spec_path = excluded.spec_path,
        current_wave = excluded.current_wave,
        active_task_id = excluded.active_task_id,
        last_review_result = excluded.last_review_result,
        updated_at = excluded.updated_at
        "#,
            params![
                input.plan_id,
                input.plan_path,
                input.spec_path,
                input.current_wave,
                input.active_task_id,
                input.last_review_result,
                now
            ],
        )?;
        self.require_plan_state(&input.plan_id)
    }

    pub fn upsert_task_state(&self, input: TaskStateUpsertInput) -> Result<TaskStateRecord> {
        let now = now_iso();
        self.conn.execute(
            r#"
      INSERT INTO task_state (
        plan_id, task_id, category_id, status, active_step_label, assigned_role, agent_id, write_lease_id,
        spec_review_status, quality_review_status, retry_count, blocker_type, blocker_message, updated_at
      ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)
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
        "#,
            params![
                input.plan_id,
                input.task_id,
                input.category_id,
                input.status,
                input.active_step_label,
                input.assigned_role,
                input.agent_id,
                input.write_lease_id,
                input.spec_review_status,
                input.quality_review_status,
                input.retry_count,
                input.blocker_type,
                input.blocker_message,
                now
            ],
        )?;
        self.require_task_state(&input.plan_id, &input.task_id)
    }

    pub fn record_task_run(
        &self,
        plan_id: &str,
        task_id: &str,
        role: &str,
        agent_id: &str,
        status: &str,
        summary: Option<&str>,
    ) -> Result<()> {
        self.conn.execute(
            r#"
      INSERT INTO task_run (plan_id, task_id, role, agent_id, status, summary, started_at)
      VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
        "#,
            params![plan_id, task_id, role, agent_id, status, summary, now_iso()],
        )?;
        Ok(())
    }

    pub fn record_evidence(
        &self,
        plan_id: &str,
        task_id: &str,
        kind: &str,
        command: Option<&str>,
        result: Option<&str>,
        summary: Option<&str>,
    ) -> Result<()> {
        self.conn.execute(
            r#"
      INSERT INTO verification_evidence (plan_id, task_id, kind, command, result, summary, created_at)
      VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
        "#,
            params![plan_id, task_id, kind, command, result, summary, now_iso()],
        )?;
        Ok(())
    }

    pub fn list_evidence_for_task(&self, plan_id: &str, task_id: &str) -> Result<Vec<EvidenceRecord>> {
        let mut stmt = self.conn.prepare(
            r#"
      SELECT
        kind,
        command,
        result,
        summary,
        created_at
      FROM verification_evidence
      WHERE plan_id = ?1 AND task_id = ?2
      ORDER BY created_at ASC
        "#,
        )?;
        let rows = stmt.query_map(params![plan_id, task_id], |row| {
            Ok(EvidenceRecord {
                kind: row.get(0)?,
                command: row.get(1)?,
                result: row.get(2)?,
                summary: row.get(3)?,
                created_at: row.get(4)?,
            })
        })?;
        rows.collect::<std::result::Result<Vec<_>, _>>().map_err(Into::into)
    }

    pub fn get_plan_state(&self, plan_id: &str) -> Result<Option<PlanStateRecord>> {
        self.conn
            .query_row(
                r#"
      SELECT
        plan_id,
        plan_path,
        spec_path,
        current_wave,
        active_task_id,
        last_review_result,
        updated_at
      FROM plan_state WHERE plan_id = ?1
                "#,
                params![plan_id],
                |row| {
                    Ok(PlanStateRecord {
                        plan_id: row.get(0)?,
                        plan_path: row.get(1)?,
                        spec_path: row.get(2)?,
                        current_wave: row.get(3)?,
                        active_task_id: row.get(4)?,
                        last_review_result: row.get(5)?,
                        updated_at: row.get(6)?,
                    })
                },
            )
            .optional()
            .map_err(Into::into)
    }

    pub fn get_task_state(&self, plan_id: &str, task_id: &str) -> Result<Option<TaskStateRecord>> {
        self.conn
            .query_row(
                r#"
      SELECT
        plan_id,
        task_id,
        category_id,
        status,
        active_step_label,
        assigned_role,
        agent_id,
        write_lease_id,
        spec_review_status,
        quality_review_status,
        retry_count,
        blocker_type,
        blocker_message,
        updated_at
      FROM task_state WHERE plan_id = ?1 AND task_id = ?2
                "#,
                params![plan_id, task_id],
                |row| {
                    Ok(TaskStateRecord {
                        plan_id: row.get(0)?,
                        task_id: row.get(1)?,
                        category_id: row.get(2)?,
                        status: row.get(3)?,
                        active_step_label: row.get(4)?,
                        assigned_role: row.get(5)?,
                        agent_id: row.get(6)?,
                        write_lease_id: row.get(7)?,
                        spec_review_status: row.get(8)?,
                        quality_review_status: row.get(9)?,
                        retry_count: row.get(10)?,
                        blocker_type: row.get(11)?,
                        blocker_message: row.get(12)?,
                        updated_at: row.get(13)?,
                    })
                },
            )
            .optional()
            .map_err(Into::into)
    }

    pub fn list_stalled_tasks(&self, older_than_ms: i64) -> Result<Vec<TaskStateRecord>> {
        let threshold = (Utc::now() - chrono::Duration::milliseconds(older_than_ms)).to_rfc3339();
        let mut stmt = self.conn.prepare(
            r#"
      SELECT
        plan_id,
        task_id,
        category_id,
        status,
        active_step_label,
        assigned_role,
        agent_id,
        write_lease_id,
        spec_review_status,
        quality_review_status,
        retry_count,
        blocker_type,
        blocker_message,
        updated_at
      FROM task_state
      WHERE updated_at < ?1 AND status NOT IN ('accepted', 'cancelled', 'blocked')
        "#,
        )?;
        let rows = stmt.query_map(params![threshold], |row| {
            Ok(TaskStateRecord {
                plan_id: row.get(0)?,
                task_id: row.get(1)?,
                category_id: row.get(2)?,
                status: row.get(3)?,
                active_step_label: row.get(4)?,
                assigned_role: row.get(5)?,
                agent_id: row.get(6)?,
                write_lease_id: row.get(7)?,
                spec_review_status: row.get(8)?,
                quality_review_status: row.get(9)?,
                retry_count: row.get(10)?,
                blocker_type: row.get(11)?,
                blocker_message: row.get(12)?,
                updated_at: row.get(13)?,
            })
        })?;
        rows.collect::<std::result::Result<Vec<_>, _>>().map_err(Into::into)
    }

    pub fn acquire_write_lease(
        &self,
        plan_id: &str,
        task_id: &str,
        holder_agent_id: &str,
        scope: &[String],
    ) -> Result<WriteLeaseRecord> {
        if self.get_active_write_lease(plan_id, task_id)?.is_some() {
            return Err(anyhow!("Active write lease already exists for {task_id}"));
        }
        let lease_id = format!("lease_{}", Uuid::new_v4());
        let created_at = now_iso();
        let scope_json = serde_json::to_string(scope)?;
        self.conn.execute(
            r#"
      INSERT INTO write_lease (lease_id, plan_id, task_id, holder_agent_id, scope_json, status, created_at, released_at)
      VALUES (?1, ?2, ?3, ?4, ?5, 'active', ?6, NULL)
        "#,
            params![lease_id, plan_id, task_id, holder_agent_id, scope_json, created_at],
        )?;
        self.require_write_lease(&lease_id)
    }

    pub fn release_write_lease(&self, lease_id: &str) -> Result<WriteLeaseRecord> {
        self.conn.execute(
            r#"
      UPDATE write_lease
      SET status = 'released', released_at = ?1
      WHERE lease_id = ?2 AND status = 'active'
        "#,
            params![now_iso(), lease_id],
        )?;
        self.require_write_lease(lease_id)
    }

    pub fn get_write_lease(&self, lease_id: &str) -> Result<Option<WriteLeaseRecord>> {
        self.conn
            .query_row(
                r#"
      SELECT
        lease_id,
        plan_id,
        task_id,
        holder_agent_id,
        scope_json,
        status,
        created_at,
        released_at
      FROM write_lease WHERE lease_id = ?1
                "#,
                params![lease_id],
                |row| {
                    Ok(WriteLeaseRecord {
                        lease_id: row.get(0)?,
                        plan_id: row.get(1)?,
                        task_id: row.get(2)?,
                        holder_agent_id: row.get(3)?,
                        scope_json: row.get(4)?,
                        status: row.get(5)?,
                        created_at: row.get(6)?,
                        released_at: row.get(7)?,
                    })
                },
            )
            .optional()
            .map_err(Into::into)
    }

    pub fn get_active_write_lease(&self, plan_id: &str, task_id: &str) -> Result<Option<WriteLeaseRecord>> {
        self.conn
            .query_row(
                r#"
      SELECT
        lease_id,
        plan_id,
        task_id,
        holder_agent_id,
        scope_json,
        status,
        created_at,
        released_at
      FROM write_lease
      WHERE plan_id = ?1 AND task_id = ?2 AND status = 'active'
      ORDER BY created_at DESC
      LIMIT 1
                "#,
                params![plan_id, task_id],
                |row| {
                    Ok(WriteLeaseRecord {
                        lease_id: row.get(0)?,
                        plan_id: row.get(1)?,
                        task_id: row.get(2)?,
                        holder_agent_id: row.get(3)?,
                        scope_json: row.get(4)?,
                        status: row.get(5)?,
                        created_at: row.get(6)?,
                        released_at: row.get(7)?,
                    })
                },
            )
            .optional()
            .map_err(Into::into)
    }

    fn require_plan_state(&self, plan_id: &str) -> Result<PlanStateRecord> {
        self.get_plan_state(plan_id)?
            .ok_or_else(|| anyhow!("Plan state missing after upsert: {plan_id}"))
    }

    fn require_task_state(&self, plan_id: &str, task_id: &str) -> Result<TaskStateRecord> {
        self.get_task_state(plan_id, task_id)?
            .ok_or_else(|| anyhow!("Task state missing after upsert: {plan_id}/{task_id}"))
    }

    fn require_write_lease(&self, lease_id: &str) -> Result<WriteLeaseRecord> {
        self.get_write_lease(lease_id)?
            .ok_or_else(|| anyhow!("Write lease missing after update: {lease_id}"))
    }
}

fn now_iso() -> String {
    Utc::now().to_rfc3339()
}
