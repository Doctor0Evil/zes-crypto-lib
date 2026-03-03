//! Key Derivation Function - Offline-first key derivation from snapshots

use serde::{Deserialize, Serialize};
use crate::error::CryptoError;
use hkdf::Hkdf;
use sha3::Sha3_256;

/// Offline KDF for snapshot-based key derivation
pub struct OfflineKDF;

impl OfflineKDF {
    /// Derive key from snapshot hash
    pub fn derive_from_snapshot(snapshot_hash: &[u8], salt: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let hk = Hkdf::<Sha3_256>::new(Some(salt), snapshot_hash);
        let mut okm = [0u8; 32];
        hk.expand(b"aln-sovereign-key", &mut okm)?;
        Ok(okm.to_vec())
    }

    /// Derive key from DID + snapshot
    pub fn derive_from_did(did: &str, snapshot_hash: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let salt = did.as_bytes();
        Self::derive_from_snapshot(snapshot_hash, salt)
    }

    /// Derive key with Argon2 for password-based scenarios
    pub fn derive_argon2(password: &[u8], salt: &[u8]) -> Result<Vec<u8>, CryptoError> {
        use argon2::{Argon2, Params};
        
        let argon2 = Argon2::default();
        let mut key = [0u8; 32];
        argon2.hash_password_into(password, salt, &mut key)?;
        Ok(key.to_vec())
    }
}

/// KDF configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KDFConfig {
    pub algorithm: String,
    pub iterations: u32,
    pub memory_kb: u32,
    pub parallelism: u32,
}

impl Default for KDFConfig {
    fn default() -> Self {
        Self {
            algorithm: "HKDF-SHA3-256".to_string(),
            iterations: 3,
            memory_kb: 65536,
            parallelism: 4,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kdf_derivation() {
        let snapshot_hash = vec![1u8; 32];
        let salt = b"test-salt";
        
        let key = OfflineKDF::derive_from_snapshot(&snapshot_hash, salt);
        assert!(key.is_ok());
        assert_eq!(key.unwrap().len(), 32);
    }
}
