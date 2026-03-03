//! Hex-Stamp Attestation - Deterministic integrity proofs

use serde::Serialize;
use sha3::{Digest, Sha3_256};
use crate::error::CryptoError;

/// Generate hex-stamp for any serializable data
pub fn generate_hex_stamp<T: Serialize>(data: &T) -> String {
    let serialized = serde_json::to_vec(data).unwrap_or_default();
    let hash = Sha3_256::digest(&serialized);
    format!("0x{}", hex::encode(hash))
}

/// Verify hex-stamp
pub fn verify_hex_stamp<T: Serialize>(data: &T, stamp: &str) -> bool {
    let expected = generate_hex_stamp(data);
    expected == stamp
}

/// Hex-stamp metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HexStampMetadata {
    pub stamp: String,
    pub timestamp: i64,
    pub algorithm: String,
    pub ledger_reference: Option<String>,
}

impl HexStampMetadata {
    pub fn new(stamp: String, ledger_ref: Option<String>) -> Self {
        Self {
            stamp,
            timestamp: chrono::Utc::now().timestamp(),
            algorithm: "SHA3-256".to_string(),
            ledger_reference: ledger_ref,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_stamp_generation() {
        let data = "test data";
        let stamp = generate_hex_stamp(&data);
        assert!(stamp.starts_with("0x"));
        assert_eq!(stamp.len(), 66);
    }

    #[test]
    fn test_hex_stamp_verification() {
        let data = "test data";
        let stamp = generate_hex_stamp(&data);
        assert!(verify_hex_stamp(&data, &stamp));
    }
}
