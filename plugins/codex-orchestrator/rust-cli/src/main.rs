use std::{
    io::{self, BufRead, Write},
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Result};
use serde::Deserialize;
use serde_json::{json, Map, Value};

use codex_orchestrator_mcp::{
    category_registry::CategoryRegistry,
    runtime_store::RuntimeStore,
    tools::{handle_tool_call, tool_specs, AppContext},
};

#[derive(Debug, Deserialize)]
struct JsonRpcRequest {
    id: Option<Value>,
    method: String,
    params: Option<Value>,
}

fn main() -> Result<()> {
    let plugin_root = resolve_plugin_root()?;
    let categories_path = plugin_root.join("config").join("categories.toml");
    let db_path = plugin_root
        .join(".codex-orchestrator")
        .join("state")
        .join("orchestrator.db");

    let ctx = AppContext {
        categories: CategoryRegistry::from_toml(
            categories_path
                .to_str()
                .ok_or_else(|| anyhow!("invalid categories path"))?,
        )?,
        runtime_store: RuntimeStore::new(
            db_path
                .to_str()
                .ok_or_else(|| anyhow!("invalid runtime DB path"))?,
        )?,
    };

    eprintln!("codex-orchestrator MCP server running on stdio");
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    for line in stdin.lock().lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        match handle_incoming(&ctx, &line) {
            Ok(Some(response)) => {
                writeln!(stdout, "{}", serde_json::to_string(&response)?)?;
                stdout.flush()?;
            }
            Ok(None) => {}
            Err(error) => eprintln!("Invalid MCP message: {error}"),
        }
    }
    Ok(())
}

fn resolve_plugin_root() -> Result<PathBuf> {
    let cwd = std::env::current_dir()?;
    if looks_like_plugin_root(&cwd) {
        return Ok(cwd);
    }

    let exe = std::env::current_exe()?;
    for ancestor in exe.ancestors() {
        if looks_like_plugin_root(ancestor) {
            return Ok(ancestor.to_path_buf());
        }
    }

    Err(anyhow!(
        "Unable to locate codex-orchestrator plugin root from cwd {} or current executable {}",
        cwd.display(),
        exe.display()
    ))
}

fn looks_like_plugin_root(path: &Path) -> bool {
    path.join("config").join("categories.toml").is_file()
}

fn handle_incoming(ctx: &AppContext, raw: &str) -> Result<Option<Value>> {
    let parsed: Value = serde_json::from_str(raw)?;
    if let Some(entries) = parsed.as_array() {
        for entry in entries {
            if let Some(response) = handle_request(ctx, serde_json::from_value(entry.clone())?)? {
                return Ok(Some(response));
            }
        }
        return Ok(None);
    }
    handle_request(ctx, serde_json::from_value(parsed)?)
}

fn handle_request(ctx: &AppContext, message: JsonRpcRequest) -> Result<Option<Value>> {
    match message.method.as_str() {
        "initialize" => {
            let protocol_version = message
                .params
                .as_ref()
                .and_then(|params| params.get("protocolVersion"))
                .and_then(Value::as_str)
                .unwrap_or("2024-11-05");
            Ok(Some(json!({
                "jsonrpc": "2.0",
                "id": message.id,
                "result": {
                    "protocolVersion": protocol_version,
                    "capabilities": { "tools": {} },
                    "serverInfo": { "name": "codex-orchestrator", "version": "0.1.0" }
                }
            })))
        }
        "notifications/initialized" => Ok(None),
        "ping" => Ok(Some(json!({
            "jsonrpc": "2.0",
            "id": message.id,
            "result": {}
        }))),
        "tools/list" => Ok(Some(json!({
            "jsonrpc": "2.0",
            "id": message.id,
            "result": {
                "tools": tool_specs().into_iter().map(|tool| json!({
                    "name": tool.name,
                    "description": tool.description,
                    "inputSchema": tool.input_schema
                })).collect::<Vec<_>>()
            }
        }))),
        "tools/call" => {
            let params = message.params.unwrap_or_else(|| json!({}));
            let tool_name = params
                .get("name")
                .and_then(Value::as_str)
                .ok_or_else(|| anyhow!("Missing tool name"))?;
            let args = params
                .get("arguments")
                .and_then(Value::as_object)
                .cloned()
                .unwrap_or_else(Map::new);
            match handle_tool_call(ctx, tool_name, &args) {
                Ok(result) => Ok(Some(json!({
                    "jsonrpc": "2.0",
                    "id": message.id,
                    "result": result
                }))),
                Err(error) => Ok(Some(json!({
                    "jsonrpc": "2.0",
                    "id": message.id,
                    "error": { "code": -32603, "message": error.to_string() }
                }))),
            }
        }
        _ => Ok(Some(json!({
            "jsonrpc": "2.0",
            "id": message.id,
            "error": { "code": -32601, "message": format!("Unsupported method: {}", message.method) }
        }))),
    }
}
