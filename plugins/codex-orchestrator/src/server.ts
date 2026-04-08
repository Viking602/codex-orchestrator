import { resolve, dirname } from "node:path";
import { fileURLToPath } from "node:url";
import * as readline from "node:readline";
import { CategoryRegistry } from "./services/category-registry.ts";
import { RuntimeStore } from "./services/runtime-store.ts";
import { createTools } from "./tools/register-tools.ts";

const currentFile = fileURLToPath(import.meta.url);
const pluginRoot = resolve(dirname(currentFile), "..");
const categoriesPath = resolve(pluginRoot, "config/categories.toml");
const dbPath = resolve(pluginRoot, ".codex-orchestrator/state/orchestrator.db");

const categories = CategoryRegistry.fromToml(categoriesPath);
const runtimeStore = new RuntimeStore(dbPath);
const tools = createTools({ categories, runtimeStore });
const toolMap = new Map(tools.map((tool) => [tool.name, tool]));

type JsonRpcId = string | number;

type JsonRpcRequest = {
  jsonrpc: "2.0";
  id?: JsonRpcId;
  method: string;
  params?: Record<string, unknown>;
};

function send(message: unknown): void {
  process.stdout.write(`${JSON.stringify(message)}\n`);
}

function success(id: JsonRpcId, result: Record<string, unknown>): void {
  send({ jsonrpc: "2.0", id, result });
}

function failure(id: JsonRpcId | undefined, code: number, message: string): void {
  if (id === undefined) return;
  send({
    jsonrpc: "2.0",
    id,
    error: { code, message },
  });
}

async function handleRequest(message: JsonRpcRequest): Promise<void> {
  switch (message.method) {
    case "initialize": {
      const protocolVersion = typeof message.params?.protocolVersion === "string"
        ? message.params.protocolVersion
        : "2024-11-05";
      success(message.id!, {
        protocolVersion,
        capabilities: {
          tools: {},
        },
        serverInfo: {
          name: "codex-orchestrator",
          version: "0.1.0",
        },
      });
      return;
    }
    case "notifications/initialized":
      return;
    case "ping":
      success(message.id!, {});
      return;
    case "tools/list":
      success(message.id!, {
        tools: tools.map((tool) => ({
          name: tool.name,
          description: tool.description,
          inputSchema: tool.inputSchema,
        })),
      });
      return;
    case "tools/call": {
      const toolName = typeof message.params?.name === "string" ? message.params.name : undefined;
      const args = typeof message.params?.arguments === "object" && message.params.arguments !== null
        ? message.params.arguments as Record<string, unknown>
        : {};
      if (!toolName) {
        failure(message.id, -32602, "Missing tool name");
        return;
      }
      const tool = toolMap.get(toolName);
      if (!tool) {
        failure(message.id, -32601, `Unknown tool: ${toolName}`);
        return;
      }
      try {
        const result = await tool.handler(args);
        success(message.id!, result);
      } catch (error) {
        failure(message.id, -32603, error instanceof Error ? error.message : String(error));
      }
      return;
    }
    default:
      failure(message.id, -32601, `Unsupported method: ${message.method}`);
  }
}

async function handleIncoming(raw: string): Promise<void> {
  if (!raw.trim()) return;
  try {
    const parsed = JSON.parse(raw) as JsonRpcRequest | JsonRpcRequest[];
    if (Array.isArray(parsed)) {
      for (const entry of parsed) {
        await handleRequest(entry);
      }
      return;
    }
    await handleRequest(parsed);
  } catch (error) {
    console.error("Invalid MCP message:", error);
  }
}

async function main() {
  const reader = readline.createInterface({
    input: process.stdin,
    crlfDelay: Infinity,
    terminal: false,
  });
  console.error("codex-orchestrator MCP server running on stdio");
  for await (const line of reader) {
    await handleIncoming(line);
  }
}

main().catch((error) => {
  console.error("Fatal error in codex-orchestrator MCP server:", error);
  process.exit(1);
});
