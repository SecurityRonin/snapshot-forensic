# snapshot-forensic

Anomaly auditor over [`snapshot-core`](https://crates.io/crates/snapshot-core) —
emits graded `forensicnomicon::report::Finding`s for temporal/integrity anomalies
across snapshot & backup restore points (broken chains, back-dated restore points,
integrity-hash mismatches between backup generations).

Part of the [SecurityRonin](https://github.com/SecurityRonin) forensic fleet; the
`[P^H]` disk-history layer. Early-stage — see the
[roadmap](https://github.com/SecurityRonin/snapshot-forensic/blob/main/docs/roadmap.md).
