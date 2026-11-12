# Khvarenah — Blackout Edition (خوَرنه) · Anti-Censorship Client

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org)
[![Platform](https://img.shields.io/badge/platform-Linux%20·%20Windows%20·%20macOS-lightgrey.svg)](docs/DEPLOYMENT.md)
[![Status](https://img.shields.io/badge/status-in%20development-yellow.svg)](docs/ROADMAP.md)

**Khvarenah (Khovarenah / خوَرنه)** is a **zero-configuration, self-healing
internet circumvention client** engineered to survive a **total national
internet blackout** — the scenario where a government severs every direct
international route, blocks DNS, and deploys Deep Packet Inspection (DPI)
against every standard proxy protocol.

It is built for the chapterIranian National Information Network (NIN / Halal
Internet)** and similar censored environments, but the techniques — connection
racing, DNS tunneling, P2P mesh — are universal. Khvarenah **never stops
trying**: it dynamically discovers working paths, races them in parallel, and
degrades gracefully through four tiers of resilience.

---

## Why Khvarenah?

When a nation-wide blackout hits, most VPN/circumvention tools fail because
they assume *some* direct foreign route still exists. Khvarenah is designed
from the ground up for the **worst case**:

- **Direct routes severed?** It races VLESS-REALITY and Shadowsocks-2022.
- **Foreign IPs black-holed?** It tunnels through domestic ISP relays and Tor
  pluggable transports (WebTunnel / Snowflake).
- **All TCP/UDP to the outside dead?** It falls back to **multipath DNS
  tunneling** using the one channel censors rarely fully sever: the recursive
  DNS resolvers of domestic ISPs.
- **Even DNS flagged?** It bridges over a local **P2P mesh** of other
  Khvarenah nodes on the same intranet.

It is autonomous, requires no configuration, and heals itself across network
interface flaps, resolver poisoning, and silent black-holing.

---

## The Resilience Ladder

| Level | Vector | Timeout | When it's used |
|------:|--------|--------:|----------------|
| 1 | **Stealth foreign egress** — VLESS-XTLS/REALITY, Shadowsocks-2022 (AEAD) | 1.5 s | Normal operation |
| 2 | **Pluggable transports & domestic relays** — Tor/WebTunnel, Snowflake, Shir-o-Khorshid over an in-country VPS | 3.0 s | Direct routes throttled |
| 3 | **Multipath DNS tunneling** — QNAME ≤101 B, base32, low-overhead ARQ | — | All foreign TCP/UDP severed |
| 4 | **Intranet P2P mesh** — mDNS neighbor discovery + local relaying | — | Last resort, blackout |

See **[docs/PRINCIPLES.md](docs/PRINCIPLES.md)** for the full threat model and
engineering doctrine.

---

## Key engineered defenses

Khvarenah is built to defeat the specific traps Iranian DPI and network
controls use. See **[docs/ARCHITECTURE.md](docs/ARCHITECTURE.md)**.

| Threat | Defense |
|--------|---------|
| TUN routing loop (infinite capture) | `SO_MARK`/fwmark policy routing on Linux, WinTUN bypass on Windows, `VpnService.protect` on Android |
| DNS poisoning (bogus `10.10.34.34` sinks) | Cryptographic authentication of every DNS-tunnel response (ChaCha20-Poly1305 / Noise); poisoned resolvers are penalized |
| Half-open TCP hangs (silent drop) | Proactive **liveness sentinel** — 4 s of zero inbound bytes triggers a soft-break and failover |
| MTU black-holing (blocked ICMP) | TCP MSS clamped to ≤1200; DNS responses kept ≤1232 bytes to prevent fragmentation |
| Wi-Fi ⇄ cellular interface flapping | Netlink network-change events → flush resolver cache, re-probe, migrate TUN sessions |
| DNS-tunnel entropy detection | Query pacing, padding, and decoy lookups so traffic matches ordinary DNS statistics |

---

## The DNS-tunnel workhorse (Level 3)

When the blackout is total, Khvarenah shifts to **native multipath DNS
tunneling**:

- Queries are striped across a **dynamic pool of 50–150 discovered domestic
  ISP recursive resolvers** (MCI, Irancell, Shatel, Mokhaberat, Respina…).
- **≤5–7 bytes of framing overhead** per packet — no heavy KCP-style payloads.
- **QNAME length strictly capped at 101 bytes** (DPI threshold) using base32 /
  base64url without padding.
- **Low-overhead selective-repeat ARQ engine** — windowed retransmission with
  cumulative + selective (SACK) acknowledgements, handling the severe UDP
  packet loss typical of DNS paths.
- TXT/NULL queries with EDNS0 buffer negotiation to stay under the 1232-byte
  fragmentation ceiling.

---

## Status

- [x] **ARQ engine** (`src/arq.rs`) — windowed selective-repeat retransmission,
      SACK bitmap, 16-bit sequence wrap-around. Pure Rust, std-only, 8 unit
      tests covering loss/reorder/duplication/wrap-around.
- [x] **Doctrine** — `docs/PRINCIPLES.md`, `docs/ROADMAP.md`.
- [ ] Authenticated DNS-tunnel responses (ChaCha20-Poly1305 / Noise)
- [ ] TUN anti-loop routing (SO_MARK / WinTUN / VpnService)
- [ ] Liveness sentinel + racing engine integration
- [ ] MTU/MSS clamping
- [ ] Interface-flap handling via netlink
- [ ] P2P LAN mesh
- [ ] DNS-tunnel entropy shaping
- [ ] Domestic resolver scanner (CIDR probing)

See **[docs/ROADMAP.md](docs/ROADMAP.md)**.

---

## Build & test

```sh
cargo build
cargo test            # ARQ engine suite
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```

---

## Documentation

- **[docs/PRINCIPLES.md](docs/PRINCIPLES.md)** — mission, threat model,
  resilience ladder, engineering standards (start here)
- **[docs/ARCHITECTURE.md](docs/ARCHITECTURE.md)** — system design, modules,
  data flow
- **[docs/ROADMAP.md](docs/ROADMAP.md)** — planned work & doctrine gaps
- **[docs/DNS-SURVIVAL.md](docs/DNS-SURVIVAL.md)** — deep dive on DNS-tunnel
  survival techniques
- **[docs/BLACKOUT-DNS-TUNNEL-PLAN.md](docs/BLACKOUT-DNS-TUNNEL-PLAN.md)** —
  implementation plan for the Level-3 tunnel
- **[docs/USER-GUIDE-DNS-TUNNEL.md](docs/USER-GUIDE-DNS-TUNNEL.md)** — usage

---

## Related & search keywords

circumvention · anti-censorship · internet blackout · Iran · NIN · Halal
Internet · censorship resistance · DNS tunnel · DNS tunneling · V2Ray ·
VLESS · REALITY · Shadowsocks · Tor · Snowflake · WebTunnel · SOCKS5 ·
obfuscation · stealth proxy · self-healing · autonomous network · Rust.

## License

MIT © Dariush Khosrowan. See [LICENSE](LICENSE).
