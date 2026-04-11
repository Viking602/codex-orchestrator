#!/usr/bin/env bash
set -euo pipefail

SCRIPT_SOURCE="${BASH_SOURCE[0]:-$0}"
SCRIPT_DIR="$(cd "$(dirname "${SCRIPT_SOURCE}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
PLUGIN_SOURCE="${REPO_ROOT}/plugins/codex-orchestrator"

normalize_home_root() {
  if [[ -n "${USERPROFILE:-}" ]]; then
    case "${USERPROFILE}" in
      [A-Za-z]:\\*)
        printf '%s' "${USERPROFILE}" | sed -E 's#^([A-Za-z]):#/\L\1#; s#\\#/#g; s#^/([a-z])#/mnt/\1#'
        return
        ;;
      *)
        printf '%s' "${USERPROFILE}"
        return
        ;;
    esac
  fi

  printf '%s' "${HOME}"
}

HOST_HOME="$(normalize_home_root)"
CODEX_HOME="${HOST_HOME}/.codex"

MODE="link"
DRY_RUN=0
PLUGIN_HOME="${CODEX_HOME}/plugins"
PLUGIN_TARGET="${PLUGIN_HOME}/codex-orchestrator"
MARKETPLACE_PATH="${HOST_HOME}/.agents/plugins/marketplace.json"
AGENT_DIR="${CODEX_HOME}/agents"
CONFIG_PATH="${CODEX_HOME}/config.toml"
CACHE_TARGET="${PLUGIN_HOME}/cache/local-plugins/codex-orchestrator/local"
GLOBAL_AGENTS_PATH_EXPLICIT=0

set_default_global_agents_path() {
  local codex_root
  codex_root="$(dirname "${CONFIG_PATH}")"
  if [[ -f "${codex_root}/AGENTS.override.md" ]]; then
    GLOBAL_AGENTS_PATH="${codex_root}/AGENTS.override.md"
  else
    GLOBAL_AGENTS_PATH="${codex_root}/AGENTS.md"
  fi
}

set_default_global_agents_path

usage() {
  cat <<'EOF'
Usage: install-codex-orchestrator.sh [options]

Options:
  --copy                    Copy plugin files instead of creating a symlink
  --link                    Symlink plugin files (default)
  --dry-run                 Print actions without modifying the filesystem
  --plugin-home <dir>       Override plugin home directory (default: ~/.codex/plugins)
  --marketplace-path <file> Override marketplace.json path (default: ~/.agents/plugins/marketplace.json)
  --agent-dir <dir>         Override Codex agent install directory (default: ~/.codex/agents)
  --config-path <file>      Override Codex config path (default: ~/.codex/config.toml)
  --global-agents-path <file> Override active global AGENTS file (default: ~/.codex/AGENTS.md or AGENTS.override.md)
  -h, --help                Show this help
EOF
}

log() {
  printf '[install] %s\n' "$*"
}

write_codex_config() {
  local config_path="$1"
  if [[ "${DRY_RUN}" -eq 1 ]]; then
    printf '[dry-run] update codex config %s -> enable codex-orchestrator@local-plugins\n' "${config_path}"
    return
  fi

  CODEX_CONFIG_TARGET="${config_path}" node <<'EOF'
const fs = require("node:fs");
const path = require("node:path");

const configPath = process.env.CODEX_CONFIG_TARGET;
if (!configPath) {
  throw new Error("Missing CODEX_CONFIG_TARGET");
}

fs.mkdirSync(path.dirname(configPath), { recursive: true });

const sectionHeader = '[plugins."codex-orchestrator@local-plugins"]';
let text = fs.existsSync(configPath) ? fs.readFileSync(configPath, "utf8") : "";

if (text.includes(sectionHeader)) {
  const sectionIndex = text.indexOf(sectionHeader);
  const nextSectionIndex = text.indexOf("\n[", sectionIndex + sectionHeader.length);
  const endIndex = nextSectionIndex === -1 ? text.length : nextSectionIndex;
  const before = text.slice(0, sectionIndex);
  const section = text.slice(sectionIndex, endIndex);
  const after = text.slice(endIndex);

  let nextSection = section;
  if (/^\s*enabled\s*=\s*(true|false)\s*$/m.test(section)) {
    nextSection = section.replace(/^\s*enabled\s*=\s*(true|false)\s*$/m, "enabled = true");
  } else {
    nextSection = `${section.trimEnd()}\nenabled = true\n`;
  }

  text = `${before}${nextSection}${after}`;
} else {
  const suffix = text.trimEnd().length === 0 ? "" : "\n\n";
  text = `${text.trimEnd()}${suffix}${sectionHeader}\nenabled = true\n`;
}

fs.writeFileSync(configPath, `${text.replace(/\s*$/u, "")}\n`, "utf8");
EOF
}

run_cmd() {
  if [[ "${DRY_RUN}" -eq 1 ]]; then
    printf '[dry-run] %s\n' "$*"
  else
    eval "$@"
  fi
}

ensure_dir() {
  local dir="$1"
  run_cmd "mkdir -p \"${dir}\""
}

write_marketplace() {
  local target_plugin="$1"
  local marketplace_path="$2"
  if [[ "${DRY_RUN}" -eq 1 ]]; then
    printf '[dry-run] update marketplace %s -> %s\n' "${marketplace_path}" "${target_plugin}"
    return
  fi

  MARKETPLACE_TARGET="${marketplace_path}" PLUGIN_TARGET_PATH="${target_plugin}" node <<'EOF'
const fs = require("node:fs");
const path = require("node:path");

const marketplacePath = process.env.MARKETPLACE_TARGET;
const pluginPath = process.env.PLUGIN_TARGET_PATH;
if (!marketplacePath || !pluginPath) {
  throw new Error("Missing marketplace env");
}

fs.mkdirSync(path.dirname(marketplacePath), { recursive: true });

const marketplaceRoot = path.resolve(path.dirname(marketplacePath), "..", "..");
const relativePluginPath = path.relative(marketplaceRoot, pluginPath).split(path.sep).join("/");
const pluginEntryPath = relativePluginPath.startsWith("./") || relativePluginPath.startsWith("../")
  ? relativePluginPath
  : `./${relativePluginPath}`;

let data;
if (fs.existsSync(marketplacePath)) {
  data = JSON.parse(fs.readFileSync(marketplacePath, "utf8"));
} else {
  data = {
    name: "local-plugins",
    interface: {
      displayName: "Local Plugins",
    },
    plugins: [],
  };
}

if (!Array.isArray(data.plugins)) {
  data.plugins = [];
}

const entry = {
  name: "codex-orchestrator",
  source: {
    source: "local",
    path: pluginEntryPath,
  },
  policy: {
    installation: "AVAILABLE",
    authentication: "ON_INSTALL",
  },
  category: "Coding",
};

const index = data.plugins.findIndex((plugin) => plugin && plugin.name === entry.name);
if (index >= 0) {
  data.plugins[index] = entry;
} else {
  data.plugins.push(entry);
}

fs.writeFileSync(marketplacePath, `${JSON.stringify(data, null, 2)}\n`, "utf8");
EOF
}

write_global_agents_guidance() {
  local agents_path="$1"
  if [[ "${DRY_RUN}" -eq 1 ]]; then
    printf '[dry-run] update global AGENTS %s -> codex-orchestrator default workflow block\n' "${agents_path}"
    return
  fi

  CODEX_AGENTS_TARGET="${agents_path}" node <<'EOF'
const fs = require("node:fs");
const path = require("node:path");

const agentsPath = process.env.CODEX_AGENTS_TARGET;
if (!agentsPath) {
  throw new Error("Missing CODEX_AGENTS_TARGET");
}

fs.mkdirSync(path.dirname(agentsPath), { recursive: true });

const begin = "<!-- codex-orchestrator-default-workflow:begin -->";
const end = "<!-- codex-orchestrator-default-workflow:end -->";
const escapeRegex = (value) => value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
const managedBlock = `${begin}
## Codex Orchestrator Default Workflow

- When \`codex-orchestrator\` is installed and enabled, use it as the default workflow for repository tasks unless the user explicitly asks for a different workflow.
- Start with the bundled \`codex-orchestrator\` skill for feature work, bug fixes, refactors, debugging, docs changes, architecture work, and other multi-step repository tasks.
- Use the plugin MCP tools to resolve category, read the active plan, record step progress, and enforce review and completion gates.
- Generic process skills are fallback helpers after \`codex-orchestrator\` takes control or when the plugin is unavailable.
- Follow stronger repository-local \`AGENTS.md\` guidance when a repository provides it.
${end}`;

let text = fs.existsSync(agentsPath) ? fs.readFileSync(agentsPath, "utf8") : "";
const blockPattern = new RegExp(`${escapeRegex(begin)}[\\s\\S]*?${escapeRegex(end)}`, "u");

if (blockPattern.test(text)) {
  text = text.replace(blockPattern, managedBlock);
} else {
  const trimmed = text.replace(/\s*$/u, "");
  text = trimmed.length === 0 ? managedBlock : `${trimmed}\n\n${managedBlock}`;
}

fs.writeFileSync(agentsPath, `${text.replace(/\s*$/u, "")}\n`, "utf8");
EOF
}

backup_and_install_agent() {
  local source_file="$1"
  local target_file="$2"
  local mode="$3"

  if [[ "${DRY_RUN}" -eq 1 ]]; then
    printf '[dry-run] install agent %s -> %s (%s)\n' "${source_file}" "${target_file}" "${mode}"
    return
  fi

  mkdir -p "$(dirname "${target_file}")"

  if [[ -e "${target_file}" || -L "${target_file}" ]]; then
    if cmp -s "${source_file}" "${target_file}" 2>/dev/null; then
      log "agent already up to date: $(basename "${target_file}")"
      return
    fi

    local backup_root="${AGENT_DIR}/.codex-orchestrator-backups/$(date +%Y%m%d-%H%M%S)"
    mkdir -p "${backup_root}"
    mv "${target_file}" "${backup_root}/$(basename "${target_file}")"
    log "backed up existing agent to ${backup_root}/$(basename "${target_file}")"
  fi

  if [[ "${mode}" == "link" ]]; then
    ln -s "${source_file}" "${target_file}"
  else
    cp "${source_file}" "${target_file}"
  fi
}

install_plugin_dir() {
  local source_dir="$1"
  local target_dir="$2"
  local mode="$3"

  if [[ "${DRY_RUN}" -eq 1 ]]; then
    printf '[dry-run] install plugin dir %s -> %s (%s)\n' "${source_dir}" "${target_dir}" "${mode}"
    return
  fi

  mkdir -p "$(dirname "${target_dir}")"
  rm -rf "${target_dir}"

  if [[ "${mode}" == "link" ]]; then
    ln -s "${source_dir}" "${target_dir}"
  else
    cp -R "${source_dir}" "${target_dir}"
    rm -rf "${target_dir}/.codex-orchestrator/state" "${target_dir}/node_modules"
    find "${target_dir}" -name '.DS_Store' -delete
  fi
}

install_cache_dir() {
  local source_dir="$1"
  local target_dir="$2"
  local mode="$3"

  if [[ "${DRY_RUN}" -eq 1 ]]; then
    printf '[dry-run] install plugin cache %s -> %s (%s)\n' "${source_dir}" "${target_dir}" "${mode}"
    return
  fi

  mkdir -p "$(dirname "${target_dir}")"
  rm -rf "${target_dir}"

  if [[ "${mode}" == "link" ]]; then
    ln -s "${source_dir}" "${target_dir}"
  else
    cp -R "${source_dir}" "${target_dir}"
  fi
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --copy)
      MODE="copy"
      shift
      ;;
    --link)
      MODE="link"
      shift
      ;;
    --dry-run)
      DRY_RUN=1
      shift
      ;;
    --plugin-home)
      PLUGIN_HOME="$2"
      PLUGIN_TARGET="${PLUGIN_HOME}/codex-orchestrator"
      CACHE_TARGET="${PLUGIN_HOME}/cache/local-plugins/codex-orchestrator/local"
      shift 2
      ;;
    --marketplace-path)
      MARKETPLACE_PATH="$2"
      shift 2
      ;;
    --agent-dir)
      AGENT_DIR="$2"
      shift 2
      ;;
    --config-path)
      CONFIG_PATH="$2"
      if [[ "${GLOBAL_AGENTS_PATH_EXPLICIT}" -eq 0 ]]; then
        set_default_global_agents_path
      fi
      shift 2
      ;;
    --global-agents-path)
      GLOBAL_AGENTS_PATH="$2"
      GLOBAL_AGENTS_PATH_EXPLICIT=1
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "Unknown option: $1" >&2
      usage >&2
      exit 1
      ;;
  esac
done

if [[ ! -d "${PLUGIN_SOURCE}" ]]; then
  echo "Plugin source not found: ${PLUGIN_SOURCE}" >&2
  exit 1
fi

if ! command -v node >/dev/null 2>&1; then
  echo "node is required to register the marketplace entry" >&2
  exit 1
fi

log "source plugin: ${PLUGIN_SOURCE}"
log "target plugin: ${PLUGIN_TARGET}"
log "installed cache: ${CACHE_TARGET}"
log "marketplace: ${MARKETPLACE_PATH}"
log "agent dir: ${AGENT_DIR}"
log "config path: ${CONFIG_PATH}"
log "global AGENTS path: ${GLOBAL_AGENTS_PATH}"
log "mode: ${MODE}"

ensure_dir "${PLUGIN_HOME}"
ensure_dir "$(dirname "${CACHE_TARGET}")"
ensure_dir "${AGENT_DIR}"
ensure_dir "$(dirname "${MARKETPLACE_PATH}")"
ensure_dir "$(dirname "${CONFIG_PATH}")"
ensure_dir "$(dirname "${GLOBAL_AGENTS_PATH}")"

install_plugin_dir "${PLUGIN_SOURCE}" "${PLUGIN_TARGET}" "${MODE}"
install_cache_dir "${PLUGIN_TARGET}" "${CACHE_TARGET}" "${MODE}"
write_marketplace "${PLUGIN_TARGET}" "${MARKETPLACE_PATH}"
write_codex_config "${CONFIG_PATH}"
write_global_agents_guidance "${GLOBAL_AGENTS_PATH}"

for agent_file in "${PLUGIN_SOURCE}"/codex/agents/*.toml; do
  backup_and_install_agent "${agent_file}" "${AGENT_DIR}/$(basename "${agent_file}")" "${MODE}"
done

log "installation complete"
if [[ "${DRY_RUN}" -eq 1 ]]; then
  log "dry-run only; no files were changed"
else
  log "plugin installed at ${PLUGIN_TARGET}"
  log "installed cache staged at ${CACHE_TARGET}"
  log "marketplace registered at ${MARKETPLACE_PATH}"
  log "agents installed into ${AGENT_DIR}"
  log "plugin enabled in ${CONFIG_PATH}"
  log "default workflow guidance updated in ${GLOBAL_AGENTS_PATH}"
fi
