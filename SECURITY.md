# Security Policy

`snapshot-forensic` is designed to parse **untrusted snapshot and backup
container files** — including artifacts acquired from compromised or actively
hostile systems. Hostile input is the expected case, not an edge case.
Robustness against crafted containers is a core design goal, and we take reports
of crashes, hangs, or memory-safety issues seriously.

> **Status:** early-stage scaffold. No parser ships yet; this policy governs the
> parser from its first commit.

## Supported versions

| Version | Supported |
|---|---|
| 0.1.x   | ✅ — current development line, receives security fixes |
| < 0.1   | ❌ — pre-release, unsupported |

Security fixes are released against the latest published `0.1.x` line.

## Reporting a vulnerability

**Do not open a public GitHub issue for a security vulnerability.**

Report privately, by either:

- **GitHub Security Advisories** — open a private advisory on the
  [`snapshot-forensic` repository](https://github.com/SecurityRonin/snapshot-forensic/security/advisories/new), or
- **Email** — [albert@securityronin.com](mailto:albert@securityronin.com).

Please include:

- the affected version and target triple,
- a minimal reproducing snapshot/backup file or byte buffer (a fuzz corpus entry
  is ideal),
- the observed behaviour (panic, hang, excessive allocation, mis-parse) and the
  expected behaviour.

We aim to acknowledge a report within a few business days and to coordinate
disclosure once a fix is available.

## Security posture

`snapshot-forensic` is hardened against adversarial input by construction:

- **`#![forbid(unsafe_code)]`** across the whole workspace — no `unsafe`, anywhere.
- **No panics on malicious input** — every length and offset is validated
  against both the structure's declared size and the actual buffer; arithmetic
  is checked or saturating.
- **Bounded reads** — record framing, headers, and length fields are
  length-checked before use, so a crafted length field cannot drive an
  out-of-bounds read or an allocation bomb.
- **Pure auditor** — the analyzer is a side-effect-free function of
  already-decoded records: no I/O, no allocation surprises.
