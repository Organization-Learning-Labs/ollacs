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

### Running Tests

#### Rust
```bash
cd rust-core
cargo test --all-targets --all-features
```

#### Go
```bash
cd go-tools
go test ./...
```

## Contributing

1. Fork the repository and create a feature branch.
2. Make your changes in `rust-core` and/or `go-tools`.
3. Run the tests locally:
   - `cargo test --all-targets --all-features` in `rust-core`
   - `go test ./...` in `go-tools`
4. Open a pull request against `main` with a clear description of your changes.
