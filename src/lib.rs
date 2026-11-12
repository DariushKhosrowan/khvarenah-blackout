//! # Khvarenah — blackout edition
//!
//! Zero-configuration, self-healing circumvention engine designed to operate
//! during a state-level internet blackout (Iranian National Information Network
//! / "Halal Internet"). It must keep a connection alive even when all standard
//! TCP/UDP transit to international addresses is severed, falling back to
//! domestic multipath DNS tunneling and intranet mesh bridging.
//!
//! This crate is the **independent, isolated** home for the blackout-focused
//! development effort. See [`docs/PRINCIPLES.md`](../../docs/PRINCIPLES.md) for
//! the full mission, threat model, and resilience doctrine.
//!
//! # Current subsystems
//!
//! - `arq` — low-overhead selective-repeat windowed Automatic Repeat Request
//!   (ARQ) engine for the DNS tunnel. Handles packet loss, reorder, and
//!   duplication over an unreliable UDP/DNS transport, with cumulative +
//!   selective (SACK) acknowledgements and timeout-driven retransmission.

pub mod arq;

/// Re-export the ARQ engine for ergonomic use.
pub use arq::{ArqReceiver, ArqSender, RecvVerdict, Seqno};
