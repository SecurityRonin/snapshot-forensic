# 8. Positioning as the `[P^H]` disk-history layer with a unified time-indexed reader

Date: 2026-07-24
Status: Accepted

## Context

The fleet architecture (`ronin-issen/CLAUDE.md`, "Multi-Repo Architecture")
treats state-history as a cross-cutting functor `[H]` that lifts each base
navigation primitive to a time-indexed variant. The disk path `[P]` navigates
*one* filesystem state by path (name → inode → block); the disk-history lift
`[P^H]` navigates a *cohort of states over time* — the restore points captured
by snapshot and backup containers — enumerating "what the filesystem looked like
at each point in time." `snapshot-forensic` is named in that architecture as a
planned `[P^H]` member (VSS, APFS snapshots, Time Machine, btrfs).

Two design questions follow. First, the reader's core abstraction: the many
target formats differ wildly in bytes but share a shape — an ordered set of
restore points, each with a timestamp, a parent, a full/incremental/differential
type, and a reconstructable filesystem view. Second, the boundary against
existing fleet container readers (`qcow2`, `vhdx`, `vmdk`, `ewf`), which already
decode several of the same underlying disk formats.

## Decision

Position both crates as the fleet's `[P^H]` disk-history layer and center the
reader on a single, format-agnostic, time-indexed abstraction — the
`SnapshotReader` / `SnapshotInfo` / `FormatMetadata` design sketched in
`docs/FORMAT_REFERENCE.md` §11: list restore points in chronological order,
materialize the filesystem (or raw bytes) at a chosen point, and expose common
metadata (created, machine name, geometry, encryption, compression, hashes)
across every format. `snapshot-core` decodes bytes into that time-indexed stream;
`snapshot-forensic` audits the *temporal and chain* semantics on top —
the layer's distinctive value (broken chains, back-dated/out-of-order restore
points, cross-generation deletion residue), which single-state container readers
do not surface.

## Consequences

The reader presents one navigation surface (a time-indexed restore-point cohort)
regardless of whether the underlying bytes are a QCOW2 internal-snapshot table, a
VMDK delta chain, an iOS backup manifest, or a Veeam metadata bank — matching the
fleet's "one primitive per layer" model.

Two integration seams are **planned, not yet wired**, and are stated here rather
than implied:

- The fleet `[H]` rule expects disk-history crates to depend on
  `state-history-forensic` and export `TemporalCohort<H>` upward; the current
  `forensic/Cargo.toml` depends only on `snapshot-core` + `forensicnomicon`, so
  the `state-history-forensic` type-sharing is a residual to wire when the reader
  materializes real cohorts. Rationale for deferring it is reconstructed from the
  scaffold state; the original intent is not recorded in history.
- Where a fleet container reader already decodes an underlying disk format
  (`qcow2`, `vhdx`, `vmdk`, `ewf`), the fleet "prefer our own crates" +
  VFS-abstraction disciplines favor reusing it for the raw-image layer and
  keeping only the snapshot-chain/temporal logic here, rather than re-decoding.
  No such dependency is wired yet (deps are `thiserror` + `forensicnomicon`
  only); the reuse boundary is a design intent to honor as decoders land.
