# rust-example

A short Rust project with:
- A reusable library (`src/lib.rs`) with validation and overflow-safe math
- A CLI binary (`src/main.rs`) with simple subcommands
- Unit and integration tests

## Run

Greet someone:

```bash
cargo run -- greet Alice
```

Add two integers:

```bash
cargo run -- add 10 15
```

Show help:

```bash
cargo run -- help
```

## Test

```bash
cargo test
```
