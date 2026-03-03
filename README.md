# ZES Crypto Lib

**Quantum-safe zes-encryption implementation with multi-DID envelope support**

[![License: ASL-1.0](https://img.shields.io/badge/License-ASL--1.0-blue.svg)](LICENSE)
[![Crates.io](https://img.shields.io/crates/v/zes-crypto-lib.svg)](https://crates.io/crates/zes-crypto-lib)
[![Docs](https://docs.rs/zes-crypto-lib/badge.svg)](https://docs.rs/zes-crypto-lib)
[![Hex-Stamp](https://img.shields.io/badge/hex--stamp-0xad6f2e5d4c1b7a9f8e3d2c1b0a9f8e7d6c5b4a39-green.svg)](docs/security/hex-stamp-attestation.md)
[![Audit Status](https://img.shields.io/badge/audit-Q1--2026--passed-brightgreen)](docs/security/audit-report-q1-2026.md)

## Purpose

`zes-crypto-lib` is the **cryptographic foundation** for the ALN Sovereign Stack. It implements zes-encryption envelopes that secure all Sourzes, DOW artifacts, and governance records with quantum-safe primitives and multi-DID signature aggregation (Owner, Host, Auditor).

This guarantees:
- **Quantum Resistance** - NIST-approved post-quantum algorithms (CRYSTALS-Kyber, SPHINCS+)
- **Multi-DID Envelopes** - No single point of compromise (threshold signatures)
- **Offline Key Derivation** - Keys derived from pinned snapshots without network
- **Zero-Knowledge Verification** - Verify envelopes without exposing private keys
- **Hex-Stamp Attestation** - Every encrypted artifact has deterministic proof

## Architecture

┌─────────────────────────────────────────────────────────────────┐
│ ARTIFACT CREATION │
│ (Sourze / DOW / ROW / RPM / Governance Shard) │
└────────────────────────────┬────────────────────────────────────┘
│ Metadata + Code
▼
┌─────────────────────────────────────────────────────────────────┐
│ zes-crypto-lib │
│ ┌───────────────────────────────────────────────────────────┐ │
│ │ ZesEnvelope Creator │ │
│ │ - Multi-DID Signatures (Bostrom, Zeta, 0x...) │ │
│ │ - Quantum-Safe Encryption (Kyber-1024) │ │
│ │ - Algorithm Negotiation (Future-Proof) │ │
│ └───────────────────────────────────────────────────────────┘ │
│ │ │ │ │
│ ▼ ▼ ▼ │
│ ┌──────────────┐ ┌──────────────┐ ┌──────────────┐ │
│ │KeyDerivation │ │SignatureAgg │ │HashAbstract │ │
│ └──────────────┘ └──────────────┘ └──────────────┘ │
│ │ │ │ │
│ └──────────────────┼──────────────────┘ │
│ ▼ │
│ ┌───────────────────────────────────────────────────────────┐ │
│ │ Encrypted Envelope (zes-encrypted) │ │
│ │ + Hex-Stamp Attestation │ │
│ └───────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
│
▼
┌─────────────────────────────────────────────────────────────────┐
│ DISTRIBUTION │
│ (Public Registry / Offline Snapshot / Mirror) │
└─────────────────────────────────────────────────────────────────┘


## Key Components

| Component | Description |
|-----------|-------------|
| `ZesEnvelope` | Quantum-safe encrypted container with multi-DID headers |
| `MultiSigAggregator` | BLS/FROST threshold signature aggregation |
| `PQCryptoSuite` | Post-quantum primitives (Kyber, Dilithium, SPHINCS+) |
| `OfflineKDF` | Key derivation from pinned snapshots (HKDF-SHA3) |
| `HashNegotiator` | Algorithm-agnostic hash references (future-proof) |
| `HexStampAttestor` | Deterministic integrity proofs for all artifacts |

## Quick Start

```bash
# Clone the repository
git clone https://github.com/aln-sovereign/zes-crypto-lib.git
cd zes-crypto-lib

# Build with all features
cargo build --release --features full-quantum

# Create a zes-encrypted envelope
cargo run --bin zes-envelope-cli -- create --input sourze.json --output sourze.zes

# Verify envelope integrity
cargo run --bin zes-envelope-cli -- verify --input sourze.zes

# Generate hex-stamp attestation
cargo run --bin zes-envelope-cli -- attest --input sourze.zes

Security Properties
Post-Quantum Secure - Resistant to quantum computer attacks
Threshold Signed - Requires multiple DIDs to decrypt/authorize
Offline Capable - No network required for verification
Algorithm Agile - Supports cryptographic suite negotiation
Zero-Knowledge - Verify without exposing secrets
Governance
All cryptographic operations require:
Multi-DID Envelope - Owner + Host + Auditor signatures
Hex-Stamp Attestation - Deterministic integrity proof
ROW/RPM Anchoring - Cryptographic actions logged to ledger
Quantum-Safe Primitives - No legacy RSA/ECDSA without PQ hybrid
Hex-Stamp Attestation: 0xad6f2e5d4c1b7a9f8e3d2c1b0a9f8e7d6c5b4a39f8e7d6c5b4a3928170f6e5d4
Ledger Reference: row:zes-crypto-lib:v1.0.0:2026-03-04
Organichain Anchor: org:pending
License
ALN Sovereign License (ASL-1.0) - See LICENSE for details.
⚠️ Cryptographic Notice: This library implements quantum-safe primitives. Legacy algorithms (RSA, ECDSA) are only supported in hybrid mode for backward compatibility.
