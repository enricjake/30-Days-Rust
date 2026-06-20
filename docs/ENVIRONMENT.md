# 🛠️ Development Environment Configuration

## Hardware & OS
- **Hostname:** `dorkserver`
- **Client Device:** `Juice`

## Software Stack
- **Editor:** Visual Studio Code (VS Code)
- **Rust Toolchain:** Managed via `rustup` (`rustc`, `cargo`)

## AI Assistant Configuration
Your VS Code setup utilizes the **Continue.dev** extension, giving you access to local and cloud-based LLMs to help debug, explain concepts, and write idiomatic code.

### Connected Providers
1. **OpenRouter / Gemini / Cloudflare Workers AI:** Used for high-level reasoning, complex architectural questions, and code translation from JS/Python to Rust.
2. **OpenCode Zen Free:** Utilized for local auto-completion, quick inline edits, and low-latency suggestions.

> **Warning for Rust Learners:** Because Rust has a strict compiler (the "Borrow Checker"), use your AI tools primarily for *explanations* and *refactoring suggestions* rather than generating boilerplate. Fighting the compiler manually is the fastest way to build muscle memory for Rust ownership rules!