#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
PLUGIN_SOURCE="${REPO_ROOT}/plugins/codex-orchestrator"

MODE="link"
DRY_RUN=0
PLUGIN_HOME="${HOME}/plugins"
PLUGIN_TARGET="${PLUGIN_HOME}/codex-orchestrator"
MARKETPLACE_PATH="${HOME}/.agents/plugins/marketplace.json"
AGENT_DIR="${HOME}/.codex/agents"
CONFIG_PATH="${HOME}/.codex/config.toml"

usage() {
  cat <<'EOF'
Usage: install-codex-orchestrator.sh [options]

Options:
  --copy                    Copy plugin files instead of creating a symlink
  --link                    Symlink plugin files (default)
  --dry-run                 Print actions without modifying the filesystem
  --plugin-home <dir>       Override plugin home directory (default: ~/plugins)
  --marketplace-path <file> Override marketplace.json path (default: ~/.agents/plugins/marketplace.json)
  --agent-dir <dir>         Override Codex agent install directory (default: ~/.codex/agents)
  --config-path <file>      Override Codex config path (default: ~/.codex/config.toml)
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

if (/^\s*apps\s*=\s*false\s*$/m.test(text)) {
  text = text.replace(/^\s*apps\s*=\s*false\s*$/m, "apps = true");
} else if (text.includes("[features]") && !/^\s*apps\s*=\s*(true|false)\s*$/m.test(text)) {
  text = text.replace(/\[features\]\n/, "[features]\napps = true\n");
} else if (!text.includes("[features]")) {
  const prefix = text.trimEnd().length === 0 ? "" : `${text.trimEnd()}\n\n`;
  text = `${prefix}[features]\napps = true\n`;
}

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

  MARKETPLACE_TARGET="${marketplace_path}" PLUGIN_ENTRY_PATH="./plugins/codex-orchestrator" node <<'EOF'
const fs = require("node:fs");
const path = require("node:path");

const marketplacePath = process.env.MARKETPLACE_TARGET;
const pluginPath = process.env.PLUGIN_ENTRY_PATH;
if (!marketplacePath || !pluginPath) {
  throw new Error("Missing marketplace env");
}

fs.mkdirSync(path.dirname(marketplacePath), { recursive: true });

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
    path: pluginPath,
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
log "marketplace: ${MARKETPLACE_PATH}"
log "agent dir: ${AGENT_DIR}"
log "config path: ${CONFIG_PATH}"
log "mode: ${MODE}"

ensure_dir "${PLUGIN_HOME}"
ensure_dir "${AGENT_DIR}"
ensure_dir "$(dirname "${MARKETPLACE_PATH}")"
ensure_dir "$(dirname "${CONFIG_PATH}")"

install_plugin_dir "${PLUGIN_SOURCE}" "${PLUGIN_TARGET}" "${MODE}"
write_marketplace "${PLUGIN_TARGET}" "${MARKETPLACE_PATH}"
write_codex_config "${CONFIG_PATH}"

for agent_file in "${PLUGIN_SOURCE}"/codex/agents/*.toml; do
  backup_and_install_agent "${agent_file}" "${AGENT_DIR}/$(basename "${agent_file}")" "${MODE}"
done

log "installation complete"
if [[ "${DRY_RUN}" -eq 1 ]]; then
  log "dry-run only; no files were changed"
else
  log "plugin installed at ${PLUGIN_TARGET}"
  log "marketplace registered at ${MARKETPLACE_PATH}"
  log "agents installed into ${AGENT_DIR}"
  log "plugin enabled in ${CONFIG_PATH}"
fi
