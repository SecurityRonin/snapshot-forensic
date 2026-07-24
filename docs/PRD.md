# snapshot-forensic — Purpose & Scope

*A reverse-written intent document for a **library-tier** fleet repo: two
published Rust crates, no runnable binary. Every current-state claim below is
grounded in a same-session read of the repo (2026-07-24) — `Cargo.toml`
(workspace + members), both `lib.rs` files, `docs/FORMAT_REFERENCE.md`, and the
git log. The load-bearing decisions live as ADRs
[0001](decisions/0001-reader-analyzer-split.md)–[0008](decisions/0008-disk-history-layer-unified-time-indexed-reader.md)
under [`docs/decisions/`](decisions/). This is the lighter library PRD, not a
product PRD: it states what the crates are, who links them, and the scope
boundary — not a user-facing feature set, because nothing here ships a tool an
examiner runs.*

## Summary

`snapshot-forensic` is the SecurityRonin fleet's planned `[P^H]` **disk-history
layer**: temporal filesystem reconstruction from snapshot and backup containers.
Where the base disk path navigates *one* filesystem state by path, this layer
lifts that to a *time-indexed cohort* of states — the restore points captured by
APFS snapshots, Time Machine, btrfs, VSS-adjacent shadow copies, and enterprise
backup images — to reconstruct what a filesystem looked like at each point in
time and surface the temporal/integrity anomalies a casual read would miss.

It ships as **two library crates**, per the fleet reader/analyzer standard
([ADR 0001](decisions/0001-reader-analyzer-split.md)):

| Crate | Role | Depends on | Emits |
|---|---|---|---|
| `snapshot-core` (`[lib] name = "snapshot"`) | reader / decoder | `thiserror` | decoded, time-indexed restore points |
| `snapshot-forensic` | anomaly analyzer | `snapshot-core`, `forensicnomicon` | graded `forensicnomicon::report::Finding`s |

**Current state, honestly:** the repo is an early-stage scaffold. The format
research that guides the design is complete
([`docs/FORMAT_REFERENCE.md`](FORMAT_REFERENCE.md), 1085 lines); the parser is
under construction and **no decode logic ships yet** — both crates are honest
stubs behind a fully hardened workspace (lints, CI, docs site). This document
describes the intended shape, grounded in what the scaffold commits to.

## What these crates are

- **`snapshot-core` — the reader.** Decodes snapshot/backup container formats
  into an addressable, time-indexed stream of restore points. It stays pure: it
  decodes bytes and makes no forensic judgments (`core/src/lib.rs`). The intended
  abstraction is a single, format-agnostic `SnapshotReader` — list restore
  points chronologically, materialize the filesystem (or raw bytes) at a chosen
  point, expose common metadata — across every target format
  ([ADR 0008](decisions/0008-disk-history-layer-unified-time-indexed-reader.md),
  `FORMAT_REFERENCE.md` §11).
- **`snapshot-forensic` — the analyzer.** Walks the decoded restore points and
  emits severity-graded findings for temporal and integrity anomalies: broken
  snapshot chains, out-of-order or back-dated restore points, integrity-hash
  mismatches, and deletion residue across backup generations
  (`forensic/src/lib.rs`). Findings are observations, never verdicts — the
  analyst concludes ([ADR 0003](decisions/0003-findings-via-forensicnomicon-report.md)).

## Who links this

`snapshot-forensic` has no front-end of its own (no CLI, GUI, or MCP server —
this is why it is library-tier). Its consumers are other fleet code:

- **Issen (ORCHESTRATION)** aggregates `snapshot-forensic`'s findings into the
  unified fleet `Report` alongside every other artifact layer, via the shared
  `forensicnomicon::report` vocabulary.
- **Any third-party Rust tool** that needs to *read* a backup container can
  depend on `snapshot-core` alone (`use snapshot::…`), without pulling the
  analyzer or `forensicnomicon` — the reason for the reader/analyzer split and
  the low, decoupled MSRV floor
  ([ADR 0001](decisions/0001-reader-analyzer-split.md),
  [ADR 0006](decisions/0006-low-library-msrv-floor.md)).

## Scope

- **Own the temporal/chain layer.** The distinctive value is snapshot-chain and
  restore-point *time* semantics — reconstructing a filesystem at a point in
  time and grading the anomalies across generations — not one-shot single-state
  decoding.
- **A unified reader over many formats**, ranked into a 4-tier build order by
  documentation quality and forensic value
  ([ADR 0007](decisions/0007-research-first-phased-tier-build-order.md)):

  | Tier | Formats (planned) |
  |---|---|
  | Tier 1 | E01 / Ex01, VMDK, VHD / VHDX, QCOW2, raw / dd, iOS backup |
  | Tier 2 | VirtualBox VDI, AFF4, Proxmox VMA, Android ADB, tar / cpio, OCI image layers, DMG |
  | Tier 3 | Acronis TIB / TIBX, Veeam VBK, AD1, L01 / Lx01, MTF / BKF, mobile backups, Synology HBK, Datto |
  | Tier 4 (API-only) | Cohesity, Rubrik, NetApp WAFL / SnapMirror, Commvault, iCloud |

- **Hardened for untrusted input from the first parser commit.** Backup
  containers are attacker-controllable; `#![forbid(unsafe_code)]`, panic-free by
  lint, fuzzed, and validated against real artifacts plus an independent oracle
  ([ADR 0004](decisions/0004-forbid-unsafe.md),
  [ADR 0005](decisions/0005-panic-free-fuzzed-hardening.md)).

## Non-goals

- **No runnable binary.** No CLI, TUI, GUI, or MCP server. Consumption is by
  linking; the user-facing surface belongs to Issen / `disk4n6`. (This is what
  makes the repo library-tier.)
- **No forensic verdicts.** The analyzer emits observations for the analyst to
  conclude from, never legal characterizations
  ([ADR 0003](decisions/0003-findings-via-forensicnomicon-report.md)).
- **No direct parsing of Tier-4 API-only backends.** Cohesity, Rubrik, NetApp,
  Commvault, and iCloud are accessible only via vendor APIs; the forensic
  approach is API extraction to standard formats, not byte-level decode
  ([ADR 0007](decisions/0007-research-first-phased-tier-build-order.md)).
- **No re-decoding of formats a fleet container reader already owns.** Where
  `qcow2` / `vhdx` / `vmdk` / `ewf` already decode an underlying disk format, the
  intent is to reuse them for the raw-image layer and keep only the
  snapshot-chain/temporal logic here (planned, not yet wired —
  [ADR 0008](decisions/0008-disk-history-layer-unified-time-indexed-reader.md)).
- **No encryption breaking.** Encryption-gated proprietary formats (Acronis,
  Veeam, Datto, encrypted iOS/Android/Samsung/Huawei backups) are documented in
  the research; key recovery / password cracking is out of scope for these
  crates.

## Validation approach

Per the fleet Test-Data Provenance and Doer-Checker standards, each decoder — as
it lands — is validated against **real artifacts plus an independent oracle**,
not only synthetic fixtures. `docs/FORMAT_REFERENCE.md` already lines up the
oracle per format (libvmdk/QEMU for VMDK, libqcow/QEMU for QCOW2, libewf for
E01/Ex01, `iphone_backup_decrypt`/libimobiledevice for iOS, and so on), and §10
surveys the existing Rust crates that can serve as cross-checks. The panic-free
posture is proven dynamically by a `cargo-fuzz` target per parsed structure plus
a full inspect/audit `fuzz_forensic` target, built and smoke-run in CI
([ADR 0005](decisions/0005-panic-free-fuzzed-hardening.md)). No fuzz targets or
validation harness exist yet because no decode logic ships yet; they are a
prerequisite of the first decoder, not a follow-up.

## Status and residuals

- Scaffold only: hardened workspace, complete research, stub crates
  ([ADR 0007](decisions/0007-research-first-phased-tier-build-order.md)).
- `state-history-forensic` type-sharing (`TemporalCohort<H>`) is not yet a
  dependency; wiring it is a residual for when the reader materializes real
  cohorts ([ADR 0008](decisions/0008-disk-history-layer-unified-time-indexed-reader.md)).
- Reuse of fleet container readers for the raw-image layer is intended but not
  wired (deps are `thiserror` + `forensicnomicon` only).

---

[Privacy Policy](https://securityronin.github.io/snapshot-forensic/privacy/) · [Terms of Service](https://securityronin.github.io/snapshot-forensic/terms/) · © 2026 Security Ronin Ltd
