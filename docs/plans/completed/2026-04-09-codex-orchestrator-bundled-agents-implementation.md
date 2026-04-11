# Codex Orchestrator Bundled Agents Implementation Plan

> **For agentic workers:** REQUIRED WORKFLOW: Keep the active implementation plan authoritative, update routing docs in the same pass, and treat bundled agent definitions as plugin-owned files rather than chat-only decisions.

**Goal:** Bundle the plugin's default Codex agent surface with the repository so planning, research, MCP implementation, review, dispatch, and routing-doc maintenance roles ship with the plugin.

**Architecture:** The plugin remains a control-plane and state-plane system. Codex native subagents remain the execution runtime. This pass only vendors and customizes agent definitions, wires them through the plugin agent manifest, and adds drift checks.

**Tech Stack:** Markdown docs, Codex `.toml` custom agents, plugin `openai.yaml` metadata, Node.js test runner, existing TypeScript category parser.

---

## Context

- Phase 3 completed the parent-accountability runtime.
- The plugin still relies on host agent inventory for its preferred roles.
- `categories.toml` already names preferred roles that can be bundled directly with the plugin.

## Artifact Model

- Design spec: `docs/specs/2026-04-09-codex-orchestrator-bundled-agents-design.md`
- Completed plan path: `docs/plans/completed/2026-04-09-codex-orchestrator-bundled-agents-implementation.md`
- Long-lived architecture note: `docs/architecture/bundled-agent-bundle.md`
- Plugin bundle root: `plugins/codex-orchestrator/codex/agents/`

## Task Dependency Graph

| Task | Depends On | Reason |
|---|---|---|
| BA-T1. Create bundled-agent spec and execution plan | None | Establish the new execution anchor |
| BA-T2. Vendor and customize bundled Codex agents | BA-T1 | Agent content should follow the new bundle contract |
| BA-T3. Wire plugin metadata and routing docs | BA-T2 | Manifest wiring depends on the final bundle shape |
| BA-T4. Add drift tests and verification | BA-T2, BA-T3 | Validation needs the bundle and manifest wiring in place |

## Parallel Execution Graph

Wave 1:
- BA-T1. Create bundled-agent spec and execution plan

Wave 2:
- BA-T2. Vendor and customize bundled Codex agents

Wave 3:
- BA-T3. Wire plugin metadata and routing docs
- BA-T4. Add drift tests and verification

Critical Path:
- BA-T1 -> BA-T2 -> BA-T3 -> BA-T4

## Quality Gates

- The bundle must include the preferred roles for `plan`, `research`, `backend-impl`, and `review`.
- Bundled agent files must carry source attribution and plugin-specific instructions.
- `agents/openai.yaml` must advertise the codex agent bundle.
- Tests must fail if bundled inventory and category preferences diverge.

## Execution Status

- Current wave: Completed
- Active task: none
- Blockers: None
- Last review result: BA-T4 quality pass

## TODO List

- [x] BA-T1. Create Bundled-Agent Spec And Execution Plan
- [x] BA-T2. Vendor And Customize Bundled Codex Agents
- [x] BA-T3. Wire Plugin Metadata And Routing Docs
- [x] BA-T4. Add Drift Tests And Verification

### Task BA-T1: Create Bundled-Agent Spec And Execution Plan

**Files:**
- Create: `docs/specs/2026-04-09-codex-orchestrator-bundled-agents-design.md`
- Create: `docs/plans/completed/2026-04-09-codex-orchestrator-bundled-agents-implementation.md`
- Modify: `AGENTS.md`
- Modify: `docs/index.md`
- Modify: `task_plan.md`
- Modify: `progress.md`

**Category:** plan
**Owner Role:** harness-planner
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Write the bundled-agent design spec
- [x] Step 2: Write the active implementation plan
- [x] Step 3: Switch routing docs to the new execution anchor

### Task BA-T2: Vendor And Customize Bundled Codex Agents

**Files:**
- Create: `plugins/codex-orchestrator/codex/agents/harness-planner.toml`
- Create: `plugins/codex-orchestrator/codex/agents/harness-dispatch-gate.toml`
- Create: `plugins/codex-orchestrator/codex/agents/harness-evaluator.toml`
- Create: `plugins/codex-orchestrator/codex/agents/harness-doc-gardener.toml`
- Create: `plugins/codex-orchestrator/codex/agents/backend-developer.toml`
- Create: `plugins/codex-orchestrator/codex/agents/search-specialist.toml`
- Create: `plugins/codex-orchestrator/codex/agents/README.md`
- Create: `docs/architecture/bundled-agent-bundle.md`

**Category:** backend-impl
**Owner Role:** backend-developer
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Select the minimum bundled agent set from the two source trees
- [x] Step 2: Vendor plugin-owned `.toml` copies under `plugins/codex-orchestrator/codex/agents/`
- [x] Step 3: Record source, purpose, and modifiability in a long-lived architecture note

### Task BA-T3: Wire Plugin Metadata And Routing Docs

**Files:**
- Modify: `plugins/codex-orchestrator/agents/openai.yaml`
- Modify: `plugins/codex-orchestrator/skills/orchestrator/SKILL.md`
- Modify: `docs/architecture/README.md`
- Modify: `docs/index.md`
- Modify: `AGENTS.md`

**Category:** backend-impl
**Owner Role:** backend-developer
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Expose the bundled codex agents through `agents/openai.yaml`
- [x] Step 2: Update routing docs and skill docs to point at the bundle

### Task BA-T4: Add Drift Tests And Verification

**Files:**
- Create: `plugins/codex-orchestrator/tests/agent-bundle.test.ts`
- Modify: `progress.md`
- Modify: `task_plan.md`
- Modify: `findings.md`

**Category:** review
**Owner Role:** harness-evaluator
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Add a regression test for bundled-agent inventory and category alignment
- [x] Step 2: Run plugin tests and MCP smoke verification
- [x] Step 3: Record the result in session tracking docs

## Verification

Run:
```bash
npm test
node --experimental-strip-types src/server.ts <<'EOF'
{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-03-26","capabilities":{},"clientInfo":{"name":"probe","version":"0.0.0"}}}
{"jsonrpc":"2.0","id":2,"method":"tools/list"}
EOF
```

Expected:
- Bundle tests pass
- Existing plugin tests still pass
- MCP server still initializes and lists tools
