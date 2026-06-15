# Changelog

All notable changes to this project are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the crates adhere
to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased] — initial scaffold

### Added

- Fleet-standard workspace scaffold for `snapshot-forensic`, the planned
  `[P^H]` disk-history crate for temporal filesystem reconstruction from
  snapshot and backup formats (APFS snapshots, Time Machine, btrfs,
  VSS-adjacent shadow copies, and enterprise backup images).
  - Reader/analyzer split: `snapshot-core` (raw reader, `[lib] name =
    "snapshot"`) + `snapshot-forensic` (anomaly auditor over `snapshot-core`,
    emitting `forensicnomicon::report` findings).
  - Both crates are stubs pending the parser implementation — `#![forbid(unsafe_code)]`,
    no decode logic yet.
  - Paranoid-Gatekeeper workspace lints, Apache-2.0, MkDocs documentation site,
    and CI.
  - Format research complete in [`docs/FORMAT_REFERENCE.md`](docs/FORMAT_REFERENCE.md):
    enterprise backup formats, virtualization snapshots, mobile and cloud
    backups, forensic image formats, tape and NAS snapshots, a Rust-ecosystem
    survey, and a unified-trait design.

[Unreleased]: https://github.com/SecurityRonin/snapshot-forensic/commits/main
