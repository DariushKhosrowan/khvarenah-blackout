# Khvarenah — Deployment & Building

This describes how to build **Khvarenah (blackout edition)** and prepare it for
the target environment (censored networks, Iran NIN, total blackout).

## Requirements

- Rust **1.75+** (edition 2021).
- Linux/macOS/Windows with a stable toolchain.

## Build

```sh
cargo build --release
```

## Test

```sh
cargo test            # ARQ engine suite (loss/reorder/dup/wrap-around)
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```

## Cross-compiling hints

The ARQ engine is pure Rust and std-only, so it compiles for any target. When
native subsystems (TUN, netengine) land, per-platform build guides will be added
here.

## Distribution goals

- A single self-contained daemon binary.
- No configuration files required at first run — everything is discovered.
- Graceful degradation: the binary runs on 3G/4G/5G (LTE) as well as fixed
  broadband, and keeps working across interface changes.

See also **[README.md](../README.md)** and **[ARCHITECTURE.md](ARCHITECTURE.md)**.
