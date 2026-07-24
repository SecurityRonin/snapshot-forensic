//! `snapshot-core` — reader for snapshot & backup container formats.
//!
//! # What is this?
//!
//! `snapshot-core` is the planned `[P^H]` disk-history reader of the
//! SecurityRonin forensic fleet: it decodes snapshot and backup container
//! formats (APFS snapshots, Time Machine, btrfs send-streams, VSS-adjacent
//! shadow copies, and enterprise backup images) into an addressable,
//! time-indexed stream of restore points — the substrate for **temporal
//! filesystem reconstruction**.
//!
//! The reader stays pure: it decodes bytes and makes no forensic judgments.
//! Anomaly findings live in the companion [`snapshot-forensic`] analyzer.
//!
//! # Status
//!
//! Status: early-stage scaffold; see docs/FORMAT_REFERENCE.md
//!
//! The format research that guides this design is complete and lives in
//! `docs/FORMAT_REFERENCE.md` (enterprise backup formats, virtualization
//! snapshots, mobile and cloud backups, forensic image formats, tape and NAS
//! snapshots, plus a Rust-ecosystem survey and a unified-trait design). The
//! parser is under construction; no decode logic ships yet.
//!
//! [`snapshot-forensic`]: https://crates.io/crates/snapshot-forensic

#![forbid(unsafe_code)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod error;

pub use error::{Result, SnapshotError};
