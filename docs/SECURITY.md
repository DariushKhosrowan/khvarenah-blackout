# Khvarenah — Security & Threat Model

The security model of **Khvarenah (blackout edition)** is derived directly from
the real operational reality of state-level censorship and total blackout in
the Iranian National Information Network (NIN / Halal Internet).

## Threat model

| # | Threat | Consequence if unhandled |
|---|--------|--------------------------|
| 1 | **Intranet isolation (NIN)** — all direct foreign BGP routes severed; outbound TCP SYN/UDP dropped or spoofed with fake RST | No foreign egress at all |
| 2 | **Selective recursive DNS transit** — client→foreign DNS blocked, but domestic ISP resolvers still resolve foreign names | Only channel left is domestic DNS |
| 3 | **DPI hardware quirks** — QNAME >~101–110 B dropped; non-browser TLS fingerprints dropped; silent black-holing | Tunnel queries/transports die silently |
| 4 | **Resolver rate-limiting / null-routing** — per-domain query rate >5–10 QPS triggers throttling | Tunnel throughput collapses |
| 5 | **DNS poisoning** — spoofed responses inject domestic IP sinks (e.g. `10.10.34.34`) | Client routes traffic into a sink |
| 6 | **Interface instability** — LTE/5G cell flapping, IP changes | TUN sessions break |

## Hardening principles

- **Authenticate every DNS-tunnel response** cryptographically,ChaCha20-Poly1305 / Noise). Reject anything that fails authenticated decryption and penalize the resolver.
- **Clamp TCP MSS** ≤1200 on the TUN; keep DNS responses below 1232 bytes to avoid IP fragmentation drops.
- **Use a liveness sentinel** — if a connected route sees zero inbound bytes for 4 s during active outbound transmission, soft-break and fail over.
- **Multi-homing / interface awareness** — react to netlink changes; flush resolver cache, re-probe, migrate TUN sessions without restart.
- **Traffic must look like traffic** — pace, pad, and mix queries so DNS-tunnel traffic matches ordinary DNS statistics (defeats entropy-based ML detection).

## Trust boundaries

The client never trusts a resolver blindly — a resolver that lies is demoted.
The client never assumes a foreign route persists — it keeps racing alternatives.
The client never exposes credentials or plaintext private keys in config files
on disk.

See also **[PRINCIPLES.md](PRINCIPLES.md)**.
