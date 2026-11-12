# Khvarenah — Architecture

This document describes the system architecture of **Khvarenah (blackout
edition)**, the zero-config, self-healing internet circumvention client for
total national internet blackout (Iranian NIN / Halal Internet).

## Design goals

- **Zero configuration** — the daemon discovers everything by itself.
- **Self-healing** — it monitors, races, and degrades/re-upgrades tiers.
- **Fails last, never first** — a tier is only abandoned after its timeout AND
  its liveness sentinel both fire.
- **No single point of failure** — redundancy at every layer.

## Module layout

```
khvarenah-blackout/
├── Cargo.toml
├── src/
│   ├── lib.rs        # crate root; re-exports
│   └── arq.rs        # selective-repeat windowed ARQ (DNS tunnel)
└── docs/             # doctrine, roadmap, deep dives
```

## Subsystems

### `arq` — Automatic Repeat Request engine

The **low-overhead selective-repeat windowed ARQ** engine is the reliability
core of the DNS tunnel (Level 3). DNS transport is lossy and buffers can
reorder packets, so the engine provides:

- **Sender side** (`ArqSender`): an in-flight sliding window, sequence-number
  assignment, and timeout-driven selective retransmission. Sends are rejected
  when the window is full (`can_send`).
- **Receiver side** (`ArqReceiver`): a reorder buffer that accepts out-of-order
  frames, holds them until the gap closes, and delivers frames strictly in
  order. Cumulative + SACK acknowledgements let the sender retire exactly the
  frames the peer received.
- **16-bit sequence wrap-around** — sequence numbers are compared with serial
  arithmetic (RFC 1982 style), so the engine keeps working across the 65536
  boundary. The window is far smaller than half the sequence space, so
  wrap-around is unambiguous.

State transitions:

```
Sender:  [empty] --send--> [in-flight] --ack--> [empty]
Receiver: [expect base] --recv--> [buffer/deliver] --ack--> [advance]
```

### Future subsystems (see ROADMAP)

- `tun` — cross-platform TUN driver with anti-loop routing.
- `netengine` — connection racing (Happy Eyeballs v3).
- `dns-tunnel` — native DNS tunnel with QNAME ≤101 B cap.
- `scanner` — domestic resolver discovery (CIDR probing).
- `protocols` — VLESS/REALITY, Shadowsocks-2022, Tor adapters.
- `p2p` — intranet LAN mesh bridging.

## Concurrency & data model

`tokio` multi-threaded runtime with channel-based message passing
(`tokio::sync::mpsc`). The core decision engine is an asynchronous state
machine that owns the set of active transports and an up-to-date health table.
Lock-free/atomic routing tables keep failover seamless between direct, proxy,
and DNS-tunneled states.

## Testing strategy

- **Unit tests** — `cargo test` (ARQ suite: loss, reorder, duplication,
  wrap-around).
- **Static analysis** — `cargo clippy --all-targets -- -D warnings`.
- **Formatting** — `cargo fmt --check`.

See also **[PRINCIPLES.md](PRINCIPLES.md)** and **[ROADMAP.md](ROADMAP.md)**.
