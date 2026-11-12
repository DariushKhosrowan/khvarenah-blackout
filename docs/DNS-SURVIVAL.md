# Khvarenah — DNS-Tunnel Survival

This deep dive covers the techniques Khvarenah uses to keep a connection alive
using **DNS tunneling** as a last resort during a total internet blackout.

## Why DNS?

When all direct TCP/UDP routes to international IPs are severed, the recursive
DNS resolvers of domestic ISPs (MCI, Irancell, Shatel, Mokhaberat, Respina,
Asratec) often still retain internal recursive peering to resolve foreign
authoritative nameservers. A disciplined DNS tunnel rides this rarely-fully-cut
channel.

## Constraints that shape the design

- **QNAME length threshold** — DPI drops queries where total QNAME length
  exceeds ~101–110 bytes (RFC allows 253). We cap at **101 bytes**.
- **Platform** — payloads are encoded with base32 / base64url **without
  padding**, chunked across labels.
- **Record types** — TXT and NULL queries.
- **EDNS0** — negotiate upstream so responses stay below **1232 bytes**
  (avoids IP-level fragmentation drops).
- **Rate limits** — domestic resolvers throttle null-route queries to one
  domain beyond 5–10 QPS; we strip across a large dynamic resolver pool and
  vary nonces/names.
- **Poisoning** — we cryptographically authenticate every response; bogus
  responses (`10.10.34.34`, spoofed NXDOMAIN) are rejected and the resolver is
  penalized.

## The ARQ engine

DNS transport is unreliable and reordering. The `arq` engine adds
selective-repeat windowed retransmission with cumulative + SACK, so a lossy
DNS path still yields ordered, complete streams.

## Operational notes

Front a tunnel session, verify the resolver pool is fresh (see ROADMAP for the
domestic resolver scanner). During interface changes, flush the resolver cache
and re-probe before resuming.

See also **[BLACKOUT-DNS-TUNNEL-PLAN.md](BLACKOUT-DNS-TUNNEL-PLAN.md)** and
**[USER-GUIDE-DNS-TUNNEL.md](USER-GUIDE-DNS-TUNNEL.md)**.
