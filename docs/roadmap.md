# snapshot-forensic — R&D Roadmap

`snapshot-forensic` is the fleet's `[P^H]` disk-history layer: it lifts a single
filesystem state to a *time-indexed cohort* of states — the restore points captured
by snapshot and backup containers — and surfaces the temporal/integrity anomalies a
single-point read misses. This roadmap records what is researched, what is built, and
the order the remaining work lands in. Format-level research is captured in
[`FORMAT_REFERENCE.md`](FORMAT_REFERENCE.md); the decisions are in
[`decisions/`](decisions/); the product framing is in [`PRD.md`](PRD.md).

## Status

Early-stage: format research complete, reader/analyzer contract defined, parsers
under construction. `snapshot-core` (reader) and `snapshot-forensic` (analyzer) are
published so the names are claimed and downstream can pin the contract while the
decoders land incrementally.

## Formats in scope (research complete — see FORMAT_REFERENCE.md)

| Format | Source | Priority | Notes |
|---|---|---|---|
| **APFS snapshots** | macOS `fs_snapshot`, Time Machine local snapshots | P0 | richest temporal model; snapshot XIDs give a total order |
| **Windows VSS** | Volume Shadow Copy `System Volume Information` | P0 | most common in Windows DFIR; shadow-copy diff areas |
| **btrfs snapshots** | `btrfs subvolume snapshot`, send-streams | P1 | CoW subvolume trees; generation numbers order states |
| **Time Machine** | sparsebundle / APFS backup volumes | P1 | overlaps APFS; backup-generation chain reconstruction |
| **Enterprise backup images** | vendor backup containers | P2 | catalog-driven; per-vendor extension research pending |

## Build phases

- **Phase 0 — contract (done):** the reader (`snapshot-core`, `Read + Seek` over a
  container) / analyzer (`snapshot-forensic`, graded `forensicnomicon::report::Finding`s)
  split; the time-indexed restore-point model; the anomaly taxonomy (broken chains,
  back-dated restore points, integrity-hash mismatches across generations).
- **Phase 1 — first decoder (in progress):** the P0 reader (APFS-snapshot or VSS)
  enumerating restore points into the time-indexed cohort, validated against a real
  multi-snapshot image with an independent oracle.
- **Phase 2 — cross-generation analysis:** chain reconstruction + the temporal/integrity
  anomaly analyzer over ≥2 decoded formats.
- **Phase 3 — remaining formats:** btrfs, Time Machine, enterprise backups, each
  reconciled against a reference tool per the fleet Doer-Checker standard.

## Non-goals

- Not a general backup tool — read-only forensic reconstruction only.
- Not a filesystem reader — it navigates *restore points*; the filesystem *within* a
  restored state is handed to the appropriate `<fs>-forensic` reader.

## Validation

Each decoder ships only after reconciling restore-point counts + contents against an
independent reference (the OS's own snapshot tooling, `vssadmin`/`tmutil`/`btrfs`),
per the fleet test-data-provenance standard. Real multi-snapshot images are
gitignored + env-gated; provenance in `tests/data/README.md`.
