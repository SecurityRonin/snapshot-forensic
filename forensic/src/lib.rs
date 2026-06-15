//! `snapshot-forensic` — anomaly auditor over [`snapshot-core`] records.
//!
//! # What is this?
//!
//! `snapshot-forensic` is the planned analyzer half of the `[P^H]`
//! disk-history layer: it walks the restore points decoded by
//! [`snapshot-core`] and emits severity-graded
//! [`forensicnomicon::report::Finding`]s for temporal and integrity anomalies
//! — broken snapshot chains, out-of-order or back-dated restore points,
//! integrity-hash mismatches, and deletion residue across backup generations.
//!
//! Findings are observations, never verdicts — the analyst concludes.
//!
//! # Status
//!
//! Status: early-stage scaffold; see docs/FORMAT_REFERENCE.md
//!
//! The reader (`snapshot-core`) and this analyzer are both stubs pending the
//! parser implementation. No audit logic ships yet.
//!
//! [`snapshot-core`]: https://crates.io/crates/snapshot-core

#![forbid(unsafe_code)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]
