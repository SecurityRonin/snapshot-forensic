# 1. Reader/analyzer split — `snapshot-core` + `snapshot-forensic`

Date: 2026-07-24
Status: Accepted

## Context

`snapshot-forensic` has two jobs that pull in opposite directions. One is to
*decode* snapshot and backup container formats faithfully into an addressable,
time-indexed stream of restore points — a reader that should be reusable by any
Rust tool, forensic or not, that needs to read these formats. The other is to
render *forensic judgment* — grade broken chains, back-dated restore points, and
integrity-hash mismatches into severity-graded findings. A single crate would
force every consumer that only wants the decoder to also compile the analyzer,
and would entangle judgment logic with byte decoding.

The format research in [`docs/FORMAT_REFERENCE.md`](../FORMAT_REFERENCE.md) §11
sketched a wider decomposition (`-detect` / `-vm` / `-ewf` / `-mobile` /
`-enterprise` sub-crates). That sketch was set aside in favor of the fleet's
established two-crate reader/analyzer standard, which the whole fleet already
follows (`ntfs-forensic`, `vmdk-forensic`, `qcow2-forensic`, …); the
per-family decomposition can still happen *inside* `snapshot-core` as modules if
the surface warrants it, without a workspace-shape commitment made before a
single decoder exists.

## Decision

Structure the repo as one workspace with two members (root `Cargo.toml`
`members = ["core", "forensic"]`):

- **`core/` → crate `snapshot-core`** — the reader/decoder. Depends only on
  `thiserror`. Decodes bytes into time-indexed restore points and makes no
  forensic judgments (`core/src/lib.rs`: "The reader stays pure: it decodes
  bytes and makes no forensic judgments.").
- **`forensic/` → crate `snapshot-forensic`** — the anomaly analyzer. Depends on
  `snapshot-core` + `forensicnomicon`, and emits severity-graded findings.

This is the fleet Crate-structure standard (`ronin-issen/CLAUDE.md`,
"Crate-structure standard — reader/analyzer split"): repo named
`<x>-forensic`, member `<x>-core` reader + `<x>-forensic` analyzer.

## Consequences

A downstream Rust tool that only needs to read a backup container depends on
`snapshot-core` alone and never compiles the analyzer or `forensicnomicon`. The
analyzer stays a side-effect-free function of already-decoded records, so it
drops straight into a fleet `Report` next to every other artifact layer. The
workspace must stay acyclic (`forensic` depends on `core`, never the reverse).
The wider sub-crate decomposition from the research remains available as future
work but is not a commitment.

The fleet standard also permits `-forensic` to read *below* `-core` when the
reader's API hides an anomaly the auditor must see (raw slack, malformed fields a
robust reader normalizes). That option is left open; the initial dependency is
`snapshot-forensic → snapshot-core`, to be revisited per-audit once decoders
exist.
