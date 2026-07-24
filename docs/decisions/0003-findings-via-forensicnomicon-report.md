# 3. Analyzer emits `forensicnomicon::report` findings, never verdicts

Date: 2026-07-24
Status: Accepted

## Context

Every analyzer in the fleet used to emit a bespoke `XxxAnalysis` type, forcing
orchestration (Issen) and any future GUI to special-case N shapes. The fleet
resolved this with a single normalized reporting vocabulary,
`forensicnomicon::report` (`ronin-issen/CLAUDE.md`, "The Reporting Model"):
every analyzer converts its findings to one `Finding` / `Report` model so
consumers render them uniformly. `forensicnomicon` is the KNOWLEDGE leaf —
every analyzer depends *down* onto it and it depends on no one.

A second, load-bearing constraint applies to a forensic analyzer: findings are
*observations*, not legal conclusions. A snapshot analyzer that reports
"back-dated restore point" is stating what the timestamps show, not adjudicating
tampering — the analyst concludes.

## Decision

`snapshot-forensic` depends on `forensicnomicon` (with the `std` feature) and
emits its temporal/integrity anomalies as severity-graded
`forensicnomicon::report::Finding`s (`forensic/Cargo.toml`:
`forensicnomicon = { version = "1", features = ["std"] }`; `forensic/src/lib.rs`
documents the emitted anomaly classes — broken snapshot chains, out-of-order or
back-dated restore points, integrity-hash mismatches, deletion residue across
backup generations). The dependency direction is strictly downward:
`snapshot-forensic → { snapshot-core, forensicnomicon }`; the reader
`snapshot-core` does not depend on `forensicnomicon` and produces no findings.

Findings are framed as observations. `forensic/src/lib.rs` states it directly:
"Findings are observations, never verdicts — the analyst concludes."

## Consequences

Snapshot anomalies aggregate into the same fleet `Report` as NTFS, registry,
EVTX, and every other artifact layer, with no bespoke conversion at the
orchestration seam. The analyzer inherits `forensicnomicon`'s `Severity` /
`Category` / anomaly-`code` conventions, including the published-contract
requirement that anomaly `code`s are scheme-prefixed SCREAMING-KEBAB and never
change once shipped. Because findings stay observational, callers cannot mistake
a graded finding for a conclusion of tampering — the epistemic boundary is
preserved at the type level (a `Finding`, not a verdict).
