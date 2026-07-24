# 4. `#![forbid(unsafe_code)]`, stronger than the mmap readers' `deny` + allow

Date: 2026-07-24
Status: Accepted

## Context

The fleet's `unsafe` law (`ronin-issen/CLAUDE.md`, "Security & Robustness
Standard — Paranoid Gatekeeper", and the core "`unsafe` Is an Avoidable
Cost-Benefit Exception") sets `unsafe_code = "forbid"` as both the default and
the goal: a *provable*, badge-able "zero places a crafted input can corrupt
memory." Only a real, justified benefit warrants downgrading to
`unsafe_code = "deny"` plus a bounded per-site `#[allow]` — which is why some
readers do: `ewf` and `memory-forensic` take that exception for `memmap2::Mmap`
scanners, accepting a few pure-Rust bounded `unsafe` sites for the mmap
performance benefit.

`snapshot-core` decodes snapshot/backup containers from a byte stream and has no
established need for memory-mapping in its API contract. With no mmap benefit to
weigh, the cost-benefit exception does not apply, so the stronger posture is
available for free.

## Decision

Both crates set `#![forbid(unsafe_code)]` at the crate root (`core/src/lib.rs`,
`forensic/src/lib.rs`), backed by the workspace lint
`[workspace.lints.rust] unsafe_code = "forbid"` (root `Cargo.toml`). `forbid`
(not `deny`) is deliberate: it cannot be locally overridden by a stray
`#[allow(unsafe_code)]`, so the guarantee holds workspace-wide by construction.

If a future decoder genuinely needs one bounded `unsafe` (e.g. an mmap fast path
for multi-GB images), this decision is revisited via a new ADR that states the
benefit and why the safe alternative was rejected — following the same
`deny` + per-site `#[allow]` pattern the mmap readers use, never a silent
weakening.

## Consequences

These crates carry the strongest memory-safety posture the fleet offers and can
wear an honest `unsafe forbidden` badge (per the README badge standard, which
reserves that badge for genuinely `forbid` crates and has the mmap readers skip
it). No crafted backup container can reach a raw-pointer path. The cost is that
any future performance work wanting `unsafe` must first clear the cost-benefit
bar and downgrade to `deny` deliberately — which is the intended friction.
