# 6. Low library MSRV floor (1.81), decoupled from the 1.96 dev toolchain

Date: 2026-07-24
Status: Accepted

## Context

The fleet MSRV policy (`ronin-issen/CLAUDE.md` + `CLAUDE.core.md`, "Rust MSRV &
Toolchain Policy") separates two numbers that must not be conflated:

- The **dev toolchain** — what contributors and CI build/fmt/clippy with — is
  pinned fleet-wide to the current stable in `rust-toolchain.toml`
  (here `channel = "1.96.0"`, plus `clippy`/`rustfmt` components).
- The **declared MSRV** (`rust-version`) is a downstream-facing promise, set by
  repo *role*. Published libraries keep a low, CI-verified MSRV so they stay
  broadly consumable; raising it narrows the crates.io audience and is treated
  as near-breaking.

`snapshot-core` and `snapshot-forensic` are published libraries (ADR 0001), so
they take the library rule, not the app rule (apps declare MSRV = the pinned
toolchain because nothing pins a library dependency against them).

## Decision

Declare `rust-version = "1.81"` once in `[workspace.package]` (root
`Cargo.toml`), inherited by both members via `rust-version.workspace = true`.
This floor is deliberately far below the `1.96.0` dev pin: contributors develop
on the newest stable, but the crates only *promise* 1.81, keeping them
consumable by older toolchains. The floor is raised only if a decoder genuinely
needs a newer-Rust feature — never merely to match the toolchain.

## Consequences

Third-party Rust tools on an older toolchain can link `snapshot-core` (and the
analyzer) without upgrading. The gap between the 1.81 promise and the 1.96 dev
pin must be kept honest by a CI job that actually builds at the declared MSRV, or
the promise silently rots. Because the floor is a single workspace field, a
future bump is one edit — but it is a deliberate, audience-narrowing change, not
a drive-by.

The precise choice of **1.81** (rather than the fleet's illustrative 1.75/1.80)
was fixed at scaffold time (commit `aa83ab5`) and its exact driver — whether a
`forensicnomicon 1.x` / `thiserror 2` transitive floor or an edition-2021
feature — is not stated in the commit history. Rationale for the specific
number reconstructed from structure; original intent not recovered in available
history. The *policy* (low library floor, decoupled from the dev pin) is
fully grounded.
