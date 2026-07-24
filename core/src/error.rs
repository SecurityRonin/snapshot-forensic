//! Crate-wide error type for the snapshot/backup container reader.

/// Errors produced while decoding snapshot & backup container formats.
///
/// The reader parses untrusted, attacker-controllable container files, so it
/// never trusts a declared length and never attempts an unbounded allocation
/// (the fleet's Paranoid Gatekeeper standard). These variants surface those
/// refusals — and the underlying I/O — as loud, typed errors rather than a
/// panic or silent wrong output. Marked `#[non_exhaustive]` so format-specific
/// variants can be added as decode logic lands without a breaking change.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum SnapshotError {
    /// The input was shorter than the structure being decoded requires.
    #[error("input too short for {what}: need {need} bytes, got {got}")]
    TooShort {
        /// The structure whose decode ran out of bytes.
        what: &'static str,
        /// Bytes the structure requires.
        need: usize,
        /// Bytes actually available.
        got: usize,
    },

    /// A structure declared a size that would require an unreasonable
    /// allocation — refused rather than attempted (defends against crafted
    /// sizes / allocation bombs).
    #[error("refusing to allocate {bytes} bytes")]
    TooLarge {
        /// The refused allocation size, in bytes.
        bytes: u64,
    },

    /// An underlying I/O error from the backing `Read + Seek` source.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

/// Convenience alias for a fallible [`SnapshotError`] operation.
pub type Result<T> = std::result::Result<T, SnapshotError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn too_short_displays_and_debugs() {
        let e = SnapshotError::TooShort {
            what: "header",
            need: 16,
            got: 4,
        };
        assert_eq!(
            e.to_string(),
            "input too short for header: need 16 bytes, got 4"
        );
        assert!(format!("{e:?}").contains("TooShort"));
    }

    #[test]
    fn too_large_displays_bytes() {
        let e = SnapshotError::TooLarge { bytes: 1 << 40 };
        assert_eq!(e.to_string(), "refusing to allocate 1099511627776 bytes");
    }

    #[test]
    fn io_error_converts_and_displays() {
        let io = std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "truncated");
        let e = SnapshotError::from(io);
        assert_eq!(e.to_string(), "I/O error: truncated");
    }

    #[test]
    fn result_alias_carries_ok_and_err() {
        fn parse(ok: bool) -> Result<u8> {
            if ok {
                Ok(7)
            } else {
                Err(SnapshotError::TooLarge { bytes: 1 })
            }
        }
        assert_eq!(parse(true).unwrap(), 7);
        assert!(parse(false).is_err());
    }
}
