# 2. Publish the reader as `snapshot-core` with `[lib] name = "snapshot"`

Date: 2026-07-24
Status: Accepted

## Context

The natural import path for the reader is `use snapshot::…`, but the bare crate
name `snapshot` is already taken on crates.io — by an unrelated 2018 snapshot-
testing / golden-master crate (`crates.io/api/v1/crates/snapshot`: created
2018-08-19, categories `development-tools`, keywords `testing`/`regression`/
`golden-master`, ~4.3K downloads). It is obscure, in a different problem domain,
and safe to co-exist with; it is not a popular crate whose import path we would
be hijacking.

The fleet Crate naming grammar (`ronin-issen/CLAUDE.md`) covers exactly this
case: "If the bare `<x>` crate name is taken on crates.io by a third party we can
co-exist with safely (obscure/ours), publish `<x>-core` with `[lib] name =
"<x>"` so consumers write `use <x>::…`." The contrasting rule — keep
`<x>_core` when the bare name is a *popular* crate (as `ntfs-core` does against
Colin Finck's `ntfs`) — does not apply, because `snapshot` is obscure, not
popular.

## Decision

Publish the reader crate as **`snapshot-core`** (package name, distinct on
crates.io) but set `[lib] name = "snapshot"` in `core/Cargo.toml`, so the
published package is `snapshot-core` while consumers import it as `snapshot`.
The workspace dependency wires this explicitly:
`snapshot = { version = "0.1", path = "core", package = "snapshot-core" }`
(root `Cargo.toml` `[workspace.dependencies]`), and `snapshot-forensic` depends
on `snapshot` (the lib name) resolving to the `snapshot-core` package.

## Consequences

The package name is unambiguous and collision-free on crates.io, while the
ergonomic `use snapshot::…` import is preserved. The indirection is a one-line
`package = "snapshot-core"` in each dependent's manifest and is invisible at the
call site. If the analyzer or a downstream ever needs to disambiguate at the
package level (e.g. `cargo tree`), the `-core` package name makes the role
explicit ("the core of the `snapshot-forensic` suite").
