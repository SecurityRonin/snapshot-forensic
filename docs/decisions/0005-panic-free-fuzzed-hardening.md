# 5. Panic-free-by-lint and fuzzed hardening for untrusted backup containers

Date: 2026-07-24
Status: Accepted

## Context

Snapshot and backup containers are untrusted, attacker-controllable input. A
`.vbk`, `.tibx`, `.qcow2`, or iOS `Manifest.db` handed to the analyst may be
malformed by accident or crafted to attack the parser: length fields that lie,
truncated records, offsets past end-of-file, allocation-bomb counts. A forensic
tool that panics on a crafted artifact is a denial-of-service on the
investigation; one that silently emits wrong output is worse. The fleet's
Paranoid Gatekeeper standard (`ronin-issen/CLAUDE.md`) makes this a hard
requirement for every `*-core` / `*-forensic` crate, paired with the global
panic-free lint recipe.

## Decision

Adopt the fleet panic-free posture from the first parser commit, statically now
and dynamically as decoders land:

- **Static (in force at the scaffold):** the workspace denies
  `clippy::unwrap_used` and `clippy::expect_used` (root `Cargo.toml`
  `[workspace.lints.clippy]`, both `deny`), with `correctness` and `suspicious`
  also `deny`. Tests opt out via
  `#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]` (both
  `lib.rs`) and `clippy.toml` (`allow-unwrap-in-tests`,
  `allow-expect-in-tests`). Every integer field read will route through the
  fleet's audited `safe-read` crate rather than a hand-rolled `bytes.rs`;
  length/offset/count fields from the image are range-checked before use and
  allocations capped.
- **Dynamic (as decoders land):** each parsed structure gets a `cargo-fuzz`
  target plus a full inspect/audit `fuzz_forensic` target, built and smoke-run
  in CI, with the invariant that no input may panic. No `fuzz/` targets exist
  yet because no decode logic ships yet (`core/src/lib.rs` status: "The parser
  is under construction; no decode logic ships yet.").

The robustness claim is stated per the fleet wording standard — the measured,
tier-1 differentiator is "fuzzed"; "panic-free" appears only as the qualified
static half ("panic-free by lint"), never as a bare absolute (README "Trust but
verify").

## Consequences

Malformed evidence degrades to an error or a partial result, never a crash or a
raw-pointer path. The static lints require more verbose, bounds-checked decode
code than a quick `unwrap` would. The fuzz targets become part of the maintained
surface and a publish-gate prerequisite. Reusing `safe-read` avoids the recurring
DRY-plus-robustness failure of per-crate `read_uNN_le` copies that drift and can
overflow `usize`.
