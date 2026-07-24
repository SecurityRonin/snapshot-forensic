# 7. Research-first; scaffold before decode; phased 4-tier build order

Date: 2026-07-24
Status: Accepted

## Context

Snapshot and backup formats are a sprawling, uneven target space: some are
openly specified (QCOW2, MS-VHDX, tar), some are reverse-engineered community
knowledge (VMDK sparse extents, iOS backup keybags), and many are wholly
proprietary and encryption-gated (Acronis TIB/TIBX, Veeam VBK, Datto,
Cohesity/Rubrik API-only). Coding any parser from memory of "how the format
probably works" is how inverted bit-splits and wrong offsets ship green — the
exact failure the fleet's Research-First discipline (`CLAUDE.core.md`) exists to
prevent: find the authoritative spec, survey existing implementations, and line
up real sample data plus an independent oracle *before* the first line of
decode.

## Decision

Do the format research first and commit it, then stand up a fully hardened
scaffold, and only then write decoders — highest-value formats first. Concretely:

- **Research committed up front.** `docs/FORMAT_REFERENCE.md` (1085 lines)
  documents magic bytes, header offsets, address-translation schemes,
  encryption, existing parsers, and an independent-oracle/tool list across
  enterprise backup, virtualization snapshots, mobile/cloud backups, forensic
  image formats, tape/NAS snapshots, a Rust-ecosystem survey (§10), and a
  unified-trait design (§11).
- **Scaffold before decode.** The scaffold commit (`aa83ab5`, "scaffold
  snapshot-forensic to fleet standard") ships the reader/analyzer split,
  Paranoid-Gatekeeper lints, Apache-2.0, MkDocs site, and CI — with stub crates
  and no decode logic. Both `lib.rs` files carry an explicit status: "early-stage
  scaffold; ... no decode logic ships yet."
- **Phased 4-tier build order.** Formats are ranked by documentation quality and
  forensic value (`FORMAT_REFERENCE.md` §11, mirrored in README/`docs/index.md`):
  Tier 1 well-documented, high value (E01/Ex01, VMDK, VHD/VHDX, QCOW2, raw/dd,
  iOS backup); Tier 2 medium (VDI, AFF4, VMA, Android ADB, tar/cpio, OCI layers,
  DMG); Tier 3 proprietary/undocumented (Acronis, Veeam, AD1, L01/Lx01, MTF/BKF,
  Synology HBK, Datto); Tier 4 API-only, no direct parsing (Cohesity, Rubrik,
  NetApp, Commvault, iCloud).

## Consequences

The decode work is de-risked before it starts: each parser has a cited spec and a
named oracle to validate against (Doer-Checker), and build order follows value,
not novelty. The cost is that the repo currently ships as a scaffold whose crates
are honest stubs — a state the README, `docs/index.md`, `CHANGELOG.md`, and both
`lib.rs` files state plainly rather than overselling. Tier 4 formats are scoped
as API-extraction-to-standard-formats, not direct parsing, so the reader is not
expected to decode them at all.
