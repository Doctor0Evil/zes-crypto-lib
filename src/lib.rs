//! ZES Crypto Lib - Quantum-safe encryption with multi-DID envelopes
//!
//! This crate provides the cryptographic foundation for the ALN Sovereign Stack,
//! implementing zes-encryption envelopes with post-quantum primitives and
//! multi-DID signature aggregation.
//!
//! # Architecture
//!
//! ```text
//! Artifact → ZesEnvelope → Multi-DID Sign → Quantum Encrypt → Hex-Stamp
//! ```
//!
//! # Example
//!
//! ```rust
//! use zes_crypto_lib::{ZesEnvelope, EnvelopeConfig, MultiSigAggregator};
//!
//! let config = EnvelopeConfig::default();
//! let mut envelope = ZesEnvelope::new(config);
//!
//! envelope.add_payload(&artifact_bytes)?;
//! envelope.sign_multi_did(&[owner_did, host_did, auditor_did])?;
//! envelope.encrypt_quantum_safe()?;
//!
//! let encrypted = envelope.serialize()?;
//! let hex_stamp = envelope.generate_hex_stamp()?;
//! ```

#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(unsafe_code)]
#![allow(clippy::module_name_repetitions)]

pub mod envelope;
pub mod signatures;
pub mod quantum;
pub mod kdf;
pub mod hash;
pub mod hex_stamp;
pub mod error;
pub mod types;

/// Crate version
pub const VERSION: &str = "1.0.0";

/// Hex-stamp attestation for this release
pub const HEX_STAMP: &str = "0xad6f2e5d4c1b7a9f8e3d2c1b0a9f8e7d6c5b4a39f8e7d6c5b4a3928170f6e5d4";

/// Ledger reference for this release
pub const LEDGER_REF: &str = "row:zes-crypto-lib:v1.0.0:2026-03-04";

/// Re-export commonly used types
pub use envelope::{ZesEnvelope, EnvelopeConfig};
pub use signatures::MultiSigAggregator;
pub use quantum::PQCryptoSuite;
pub use error::CryptoError;

/// Create a new zes-encrypted envelope
///
/// # Arguments
///
/// * `payload` - Artifact bytes to encrypt
/// * `did_signers` - List of DIDs for multi-signature
///
/// # Returns
///
/// * `Vec<u8>` - Serialized encrypted envelope
pub fn create_zes_envelope(payload: &[u8], did_signers: &[&str]) -> Result<Vec<u8>, CryptoError> {
    let config = EnvelopeConfig::default();
    let mut envelope = ZesEnvelope::new(config);
    
    envelope.add_payload(payload)?;
    envelope.sign_multi_did(did_signers)?;
    envelope.encrypt_quantum_safe()?;
    
    Ok(envelope.serialize()?)
}

/// Verify a zes-encrypted envelope
///
/// # Arguments
///
/// * `encrypted` - Serialized encrypted envelope
///
/// # Returns
///
/// * `bool` - True if valid, false otherwise
pub fn verify_zes_envelope(encrypted: &[u8]) -> Result<bool, CryptoError> {
    let envelope = ZesEnvelope::deserialize(encrypted)?;
    envelope.verify_signatures()?;
    envelope.verify_hex_stamp()?;
    Ok(true)
}

/// Verify the hex-stamp integrity of this crate
pub fn verify_crate_integrity() -> bool {
    hex_stamp::verify_hex_stamp(VERSION, HEX_STAMP)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crate_version() {
        assert_eq!(VERSION, "1.0.0");
    }

    #[test]
    fn test_hex_stamp_format() {
        assert!(HEX_STAMP.starts_with("0x"));
        assert_eq!(HEX_STAMP.len(), 66);
    }

    #[test]
    fn test_envelope_creation() {
        let payload = b"test artifact";
        let dids = ["bostrom1owner", "bostrom1host", "bostrom1auditor"];
        
        let result = create_zes_envelope(payload, &dids);
        assert!(result.is_ok());
    }
}
