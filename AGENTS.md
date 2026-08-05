# AI Agent Development Guidelines (`equator-contracts`)

This repository is configured for an **AI-Native Stellar Development Workflow**.

---

## 🛠 Active AI Resources & Tools

### 1. Raven Remote MCP Server (`https://raven.stellar.buzz`)
This repository includes `.mcp.json` connecting your AI assistant to **Raven**, the remote Stellar Model Context Protocol (MCP) server.
* Use `search` to query live Stellar documentation, Soroban SDK patterns, and security guidelines.
* Use `execute` to query live ecosystem network state and testnet accounts.

### 2. Stellar Developer Skills
Official Stellar developer skills are located in `.github/skills/`:
* 🧠 `smart-contracts`: Guidelines for writing, testing (`soroban-sdk::testutils`), securing, and compiling WASM smart contracts.
* 🧠 `assets`: Guidelines for handling Stellar Native Assets, SAC (Soroban Asset Contract), and USDC trustlines.

### 3. Documentation Context (`llms.txt`)
AI assistants can reference the structured documentation index at:
* 🌐 `https://developers.stellar.org/llms.txt`

---

## 🤖 Rules for AI Assistants Working on Contracts

1. **Soroban No-Std Compliance:** Smart contracts must be annotated with `#![no_std]`.
2. **Storage Lifetime Awareness:** Always specify storage lifetime (`instance()`, `persistent()`, `temporary()`) according to access frequency.
3. **Explicit Authorization Checks:** Always call `address.require_auth()` for any state-changing invocation modifying user balances or contract status.
4. **Testing Requirements:** Every contract method must have corresponding unit tests in `src/test.rs` utilizing `Env::default()`.
