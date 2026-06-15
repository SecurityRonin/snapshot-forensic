# snapshot-forensic

**Temporal filesystem reconstruction from snapshot & backup formats — a reader
(`snapshot-core`) plus a graded anomaly analyzer (`snapshot-forensic`).**

!!! note "Status: early-stage scaffold"
    The format research that guides this design is complete (see
    [Format Reference](FORMAT_REFERENCE.md)). The parser is under construction —
    both crates currently ship as stubs.

`snapshot-forensic` is the planned `[P^H]` disk-history layer of the
SecurityRonin forensic fleet. Where the base disk path navigates one filesystem
state by path, the disk-history layer lifts that to a *time-indexed* cohort of
states: the restore points captured by snapshot and backup containers. The goal
is to reconstruct what a filesystem looked like at each point in time and to
surface the anomalies — broken chains, back-dated or out-of-order restore
points, integrity-hash mismatches, deletion residue across generations — that a
casual read would miss.

## The two-crate split

This workspace follows the fleet reader/analyzer standard:

- **`snapshot-core`** — the reader. Decodes snapshot and backup container
  formats into an addressable, time-indexed stream of restore points. No
  judgments — just bytes faithfully decoded. (`[lib] name = "snapshot"`.)
- **`snapshot-forensic`** — the analyzer. Walks the decoded restore points and
  emits severity-graded
  [`forensicnomicon::report`](https://crates.io/crates/forensicnomicon) findings
  for temporal and integrity anomalies.

The reader stays pure so it is useful on its own; all *forensic meaning* lives in
the analyzer, which drops straight into a fleet `Report` next to every other
artifact layer. Findings are observations, never verdicts — the analyst
concludes.

## Planned format coverage

The [Format Reference](FORMAT_REFERENCE.md) surveys the target formats and ranks
them. The intended build order, highest priority first:

| Tier | Formats (planned) |
|---|---|
| Tier 1 | E01 / Ex01, VMDK, VHD / VHDX, QCOW2, raw / dd, iOS backup |
| Tier 2 | VirtualBox VDI, AFF4, Proxmox VMA, Android ADB, tar / cpio, OCI image layers, DMG |
| Tier 3 | Acronis TIB / TIBX, Veeam VBK, AD1, L01 / Lx01, MTF / BKF, mobile backups, Synology HBK, Datto |
| Tier 4 (API-only) | Cohesity, Rubrik, NetApp WAFL / SnapMirror, Commvault, iCloud |

All cells above are **planned** — no decode logic ships yet.

## Trust but verify

The fleet hardening standard applies from the first parser commit: both crates
are `#![forbid(unsafe_code)]`, panic-free against attacker-controllable input,
fuzzed with `cargo-fuzz`, and validated against real artifacts and an
independent oracle. See [Format Reference](FORMAT_REFERENCE.md) for the research
and the unified-trait design.

---

[Privacy Policy](https://securityronin.github.io/snapshot-forensic/privacy/) · [Terms of Service](https://securityronin.github.io/snapshot-forensic/terms/) · © 2026 Security Ronin Ltd
