# snapshot-forensic

[![License: Apache-2.0](https://img.shields.io/badge/License-Apache--2.0-blue.svg)](LICENSE)
[![Sponsor](https://img.shields.io/badge/sponsor-h4x0r-ea4aaa?logo=github-sponsors)](https://github.com/sponsors/h4x0r)

**Temporal filesystem reconstruction from snapshot & backup formats — a panic-free-by-construction reader for snapshot/backup containers (APFS snapshots, Time Machine, btrfs, VSS-adjacent shadow copies, enterprise backup images), and a graded anomaly analyzer that flags broken chains, back-dated restore points, and integrity-hash mismatches across backup generations.**

**Status:** early-stage scaffold — format research complete (see docs), parser under construction.

`snapshot-forensic` is the planned `[P^H]` disk-history layer of the SecurityRonin forensic fleet. Where the base disk path navigates one filesystem state by path, the disk-history layer lifts that to a *time-indexed* cohort of states — the restore points captured by snapshot and backup containers — to reconstruct what a filesystem looked like at each point in time and surface the temporal/integrity anomalies a casual read would miss.

## The two-crate split

This workspace follows the fleet reader/analyzer standard:

| Crate | Role | Depends on | Emits |
|---|---|---|---|
| `snapshot-core` | reader / decoder (`[lib] name = "snapshot"`) | `thiserror` | decoded restore points (time-indexed) |
| `snapshot-forensic` | anomaly analyzer | `snapshot-core`, `forensicnomicon` | graded `Finding`s |

The reader stays pure — it decodes bytes and makes no judgments. All *forensic meaning* lives in the analyzer, which is a side-effect-free function of already-decoded records, so it drops straight into a fleet `Report` next to every other artifact layer. Findings are observations, never verdicts — the analyst concludes.

## Roadmap

The format research that guides the design is complete in [`docs/FORMAT_REFERENCE.md`](docs/FORMAT_REFERENCE.md). The intended build order, highest priority first — every format below is **planned**, no decode logic ships yet:

| Tier | Formats (planned) | Status |
|---|---|---|
| Tier 1 | E01 / Ex01, VMDK, VHD / VHDX, QCOW2, raw / dd, iOS backup | planned |
| Tier 2 | VirtualBox VDI, AFF4, Proxmox VMA, Android ADB, tar / cpio, OCI image layers, DMG | planned |
| Tier 3 | Acronis TIB / TIBX, Veeam VBK, AD1, L01 / Lx01, MTF / BKF, mobile backups, Synology HBK, Datto | planned |
| Tier 4 (API-only) | Cohesity, Rubrik, NetApp WAFL / SnapMirror, Commvault, iCloud | planned |

## Trust but verify

The fleet hardening standard applies from the first parser commit: both crates are `#![forbid(unsafe_code)]`, will be panic-free against attacker-controllable input, fuzzed with `cargo-fuzz`, and validated against real artifacts plus an independent oracle. Snapshot and backup containers are untrusted, attacker-controllable input, so the crates are designed to be hardened by construction.

## Documentation

Full format research, the unified-trait design, and the planned architecture live in [`docs/FORMAT_REFERENCE.md`](docs/FORMAT_REFERENCE.md). The MkDocs site is published to GitHub Pages.

---

[Privacy Policy](https://securityronin.github.io/snapshot-forensic/privacy/) · [Terms of Service](https://securityronin.github.io/snapshot-forensic/terms/) · © 2026 Security Ronin Ltd
