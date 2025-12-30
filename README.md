# OLL Assessment Driver

A cross-platform driver to observe and control AI tools on Linux, macOS, and Windows.

## Architecture

- **rust-core/**: Core logic, policy engine, and platform-specific drivers (Rust).
- **go-tools/**: CLI, admin tools, and integration services (Go).

## Platforms

- **Windows**: x86_64, ARM64
- **macOS**: Darwin (Intel/Apple Silicon)
- **Linux**: x86_64, ARM64

## Getting Started

### Prerequisites

- Rust (cargo)
- Go (1.21+)

### Build

#### Rust Core
```bash
cd rust-core
cargo build
```

#### Go Tools
```bash
cd go-tools
go build ./cmd/ollctl
```
