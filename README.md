# Khvarenah — Blackout Edition (خوَرنه)

A **zero-configuration, self-healing circumvention client** for total internet
blackout. Primary target: the Iranian National Information Network (NIN /
"Halal Internet") during a state-level cutoff. Khvarenah is designed to *never
stop trying*: it races connection vectors, dynamically discovers paths, and
falls back to domestic multipath DNS tunneling and intranet mesh bridging when
all direct foreign routes are severed.

> This is the **independent, isolated** home of the blackout-focused project —
> decoupled from the legacy monorepo. See `docs/PRINCIPLES.md` for the full
> mission, threat model, and engineering doctrine.

## The resilience ladder

Khvarenah executes an asynchronous, prioritized **Parallel Racing Engine**
(Happy Eyeballs on steroids). Each level has its own timeout; if it fails, the
next tier takes over — the client degrades gracefully and never gives up:

| Level | Vector | Timeout |
|-------|--------|---------|
| 1 | Stealth foreign egress (VLESS-REALITY, Shadowsocks-2022) | 1.5 s |
| 2 | Pluggable transports & domestic relays (Tor/WebTunnel, Snowflake, Shir-o-Khorshid over Iran VPS) | 3.0 s |
| 3 | Native multipath DNS tunneling (QNAME ≤101 B, base32, ARQ) | default/always |
| 4 | Intranet P2P mesh (mDNS + neighbor relay) | fallback |

## Current status

- **`arq`** — selective-repeat windowed ARQ engine for the DNS tunnel
  (sender window, receiver reorder buffer, cumulative + SACK, timeout-driven
  retransmission, 16-bit wrap-around). Pure Rust, std-only, unit-tested
  against simulated loss/reorder/duplication.
- **Doctrine** — `docs/PRINCIPLES.md` codifies the mission, threat model,
  resilience ladder, and engineering standards. `docs/ROADMAP.md` tracks gaps.

## Build & test

```sh
cargo build
cargo test          # ARQ engine suite
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```

## License

MIT © Dariush Khosrowan. See `LICENSE`.
