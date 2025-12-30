# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Architecture Overview

This repository contains the source code for the OLL Assessment Driver, a cross-platform tool to observe and control AI tools. The codebase is split into two main components:

1.  **Rust Core (`rust-core/`)**: This contains the core logic, policy engine, and platform-specific drivers. It is organized as a Cargo workspace with the following members:
    *   `core`: The business logic and policy engine.
    *   `agent`: The main executable/service that runs on the target machine.
    *   `platform-*`: Platform-specific implementations (`win`, `macos`, `linux`, `common`).

2.  **Go Tools (`go-tools/`)**: This contains the CLI and administrative tools.
    *   `cmd/ollctl`: The command-line interface tool.
    *   `internal`: Internal Go packages.

## Development Commands

### Rust Core

All Rust commands should be run from the `rust-core/` directory.

*   **Build Workspace**: `cargo build`
*   **Run Agent**: `cargo run -p agent`
*   **Test All**: `cargo test`
*   **Test Specific Package**: `cargo test -p <package_name>` (e.g., `cargo test -p core`)
*   **Lint**: `cargo clippy`
*   **Format**: `cargo fmt`

### Go Tools

All Go commands should be run from the `go-tools/` directory.

*   **Build CLI**: `go build ./cmd/ollctl`
*   **Test All**: `go test ./...`
*   **Test Specific Package**: `go test ./internal/<package>`
*   **Format**: `go fmt ./...`

## Project Structure

```text
.
├── config/             # Configuration files
├── go-tools/           # Go codebase (CLI)
│   ├── cmd/            # Entry points
│   │   └── ollctl/     # CLI tool entry point
│   └── internal/       # Internal library code
├── rust-core/          # Rust codebase (Core logic & Agent)
│   ├── agent/          # Main agent executable
│   ├── core/           # Core library
│   └── platform-*/     # Platform-specific implementations
└── README.md           # General documentation
```
