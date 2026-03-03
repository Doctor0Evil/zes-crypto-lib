//! Zes Envelope - Quantum-safe encrypted container with multi-DID headers

use serde::{Deserialize, Serialize};
use crate::error::CryptoError;
use crate::signatures::MultiSigAggregator;
use crate::quantum::PQCryptoSuite;
use crate::hex_stamp;
use uuid::Uuid;
use chrono::Utc;

/// Zes-encryption envelope structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZesEnvelope {
    /// Envelope version
    pub version: String,
    /// Unique envelope ID
    pub envelope_id: String,
    /// Creation timestamp
    pub timestamp: i64,
    /// Encrypted payload
    pub encrypted_payload: Vec<u8>,
    /// Multi-DID signatures
    pub signatures: Vec<MultiSigSignature>,
    /// Encryption algorithm used
    pub encryption_algo: String,
    /// Hash algorithm used
    pub hash_algo: String,
    /// Hex-stamp attestation
    pub hex_stamp: String,
    /// ROW/RPM reference
    pub row_reference: Option<String>,
    /// Cyberspectre trace ID
    pub cyberspectre_trace_id: Option<String>,
}

/// Multi-signature structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiSigSignature {
    pub did: String,
    pub signature: Vec<u8>,
    pub algorithm: String,
}

/// Envelope configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvelopeConfig {
    pub encryption_algo: String,
    pub hash_algo: String,
    pub require_multi_sig: bool,
    pub min_signatures: usize,
}

impl Default for EnvelopeConfig {
    fn default() -> Self {
        Self {
            encryption_algo: "CRYSTALS-Kyber-1024".to_string(),
            hash_algo: "SHA3-256".to_string(),
            require_multi_sig: true,
            min_signatures: 3, // Owner, Host, Auditor
        }
    }
}

impl ZesEnvelope {
    /// Create a new envelope
    pub fn new(config: EnvelopeConfig) -> Self {
        Self {
            version: "1.0.0".to_string(),
            envelope_id: Uuid::new_v4().to_string(),
            timestamp: Utc::now().timestamp(),
            encrypted_payload: Vec::new(),
            signatures: Vec::new(),
            encryption_algo: config.encryption_algo,
            hash_algo: config.hash_algo,
            hex_stamp: String::new(),
            row_reference: None,
            cyberspectre_trace_id: None,
        }
    }

    /// Add payload to envelope
    pub fn add_payload(&mut self, payload: &[u8]) -> Result<(), CryptoError> {
        // In production, encrypt payload here
        self.encrypted_payload = payload.to_vec();
        Ok(())
    }

    /// Sign with multiple DIDs
    pub fn sign_multi_did(&mut self, dids: &[&str]) -> Result<(), CryptoError> {
        for did in dids {
            let signature = MultiSigAggregator::sign(did, &self.encrypted_payload)?;
            self.signatures.push(signature);
        }
        Ok(())
    }

    /// Encrypt with quantum-safe algorithm
    pub fn encrypt_quantum_safe(&mut self) -> Result<(), CryptoError> {
        let suite = PQCryptoSuite::new(&self.encryption_algo)?;
        self.encrypted_payload = suite.encrypt(&self.encrypted_payload)?;
        Ok(())
    }

    /// Serialize envelope
    pub fn serialize(&mut self) -> Result<Vec<u8>, CryptoError> {
        self.generate_hex_stamp()?;
        Ok(serde_json::to_vec(self)?)
    }

    /// Deserialize envelope
    pub fn deserialize(bytes: &[u8]) -> Result<Self, CryptoError> {
        Ok(serde_json::from_slice(bytes)?)
    }

    /// Verify signatures
    pub fn verify_signatures(&self) -> Result<(), CryptoError> {
        MultiSigAggregator::verify(&self.signatures, &self.encrypted_payload)
    }

    /// Generate hex-stamp
    pub fn generate_hex_stamp(&mut self) -> Result<(), CryptoError> {
        self.hex_stamp = hex_stamp::generate_hex_stamp(self);
        Ok(())
    }

    /// Verify hex-stamp
    pub fn verify_hex_stamp(&self) -> Result<(), CryptoError> {
        if hex_stamp::verify_hex_stamp(self, &self.hex_stamp) {
            Ok(())
        } else {
            Err(CryptoError::HexStampVerificationFailed)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_envelope_lifecycle() {
        let config = EnvelopeConfig::default();
        let mut envelope = ZesEnvelope::new(config);
        
        envelope.add_payload(b"test").unwrap();
        envelope.sign_multi_did(&["did1", "did2", "did3"]).unwrap();
        envelope.encrypt_quantum_safe().unwrap();
        
        let serialized = envelope.serialize().unwrap();
        assert!(!serialized.is_empty());
    }
}
