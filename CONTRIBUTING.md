# Contributing

Thanks for your interest in contributing to the OLL Assessment Driver!

## Development setup

- Install Rust and Cargo from https://rustup.rs
- Install Go 1.21+ from https://go.dev/dl/

Clone the repository and build the components:

```bash
cd rust-core
cargo build

cd ../go-tools
go build ./cmd/ollctl
```

## Running tests

```bash
cd rust-core
cargo test --all-targets --all-features

cd ../go-tools
go test ./...
```

## Pull request guidelines

1. Create a branch off `main`.
2. Make focused, well-scoped changes.
3. Ensure Rust and Go tests pass locally.
4. Update documentation (README, CONTRIBUTING, comments) if behavior changes.
5. Open a PR with a clear summary and testing notes.
