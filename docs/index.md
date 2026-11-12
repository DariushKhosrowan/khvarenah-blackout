# Khvarenah — Blackout Edition: Documentation Index

This is the complete documentation hub for **Khvarenah**, the zero-configuration,
self-healing internet circumvention client designed to survive a **total
national internet blackout** (Iranian NIN / Halal Internet).

## Read first

| Document | Covers |
|----------|--------|
| **[README.md](../README.md)** | Project overview, resilience ladder, status, quick start |
| **[PRINCIPLES.md](PRINCIPLES.md)** | Mission, threat model, resilience ladder, engineering standards |
| **[ARCHITECTURE.md](ARCHITECTURE.md)** | System design, modules, data flow, concurrency model |
| **[ROADMAP.md](ROADMAP.md)** | Planned work and doctrine gaps |

## Deep dives

| Document | Covers |
|----------|--------|
| **[DNS-SURVIVAL.md](DNS-SURVIVAL.md)** | Survival techniques for DNS tunneling under DPI |
| **[BLACKOUT-DNS-TUNNEL-PLAN.md](BLACKOUT-DNS-TUNNEL-PLAN.md)** | Implementation plan for the Level-3 tunnel |
| **[USER-GUIDE-DNS-TUNNEL.md](USER-GUIDE-DNS-TUNNEL.md)** | Operational guide |
| **[DEPLOYMENT.md](DEPLOYMENT.md)** | Building, cross-compiling |
| **[SECURITY.md](SECURITY.md)** | Threat model, hardening |

## Project pillars

1. **Never stop trying** — the client races vectors and degrades gracefully
   through four tiers.
2. **Zero configuration** — it self-discovers paths and heals automatically.
3. **Defeat the specific traps** of Iranian DPI and NIN controls.
4. **Cryptographic integrity** — poisoned responses are rejected, not trusted.
5. **Isolated & independent** — this project is decoupled from any legacy
   codebase.

## Navigation

- [Back to README](../README.md)
- [Principles](PRINCIPLES.md)
- [Architecture](ARCHITECTURE.md)
- [Roadmap](ROADMAP.md)
