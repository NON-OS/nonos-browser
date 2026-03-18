#![deny(unsafe_code)]
#![warn(clippy::all)]

pub mod blake3_ops;
pub mod ed25519_ops;
pub mod encryption;
pub mod mnemonic;
pub mod poseidon;
pub mod poseidon_canonical;
pub mod secp256k1_ops;
pub mod stealth;
pub mod wallet_encryption;
pub mod zk_proofs;

pub use blake3_ops::*;
pub use ed25519_ops::*;
pub use encryption::*;
pub use secp256k1_ops::*;
pub use stealth::*;
pub use wallet_encryption::{
    decrypt_wallet, derive_wallet_key, encrypt_wallet, migrate_wallet, EncryptedWallet, KdfParams,
    WALLET_ENCRYPTION_VERSION,
};
// Legacy poseidon - use poseidon_canonical for new code
pub use poseidon::*;
// Canonical Poseidon implementation
pub use mnemonic::*;
pub use poseidon_canonical::{
    bytes_to_fr as canonical_bytes_to_fr, canonical_config,
    compute_nullifier as canonical_nullifier, compute_scoped_nullifier,
    fr_to_bytes as canonical_fr_to_bytes, poseidon_commitment as canonical_commitment,
    poseidon_hash as canonical_hash, poseidon_hash1_field, poseidon_hash2 as canonical_hash2,
    poseidon_hash2_fields, poseidon_hash3_fields, poseidon_hash_fields,
    PoseidonMerkleTree as CanonicalMerkleTree,
};
pub use zk_proofs::*;

pub fn random_bytes<const N: usize>() -> [u8; N] {
    use rand::RngCore;
    let mut bytes = [0u8; N];
    rand::thread_rng().fill_bytes(&mut bytes);
    bytes
}

pub fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    use subtle::ConstantTimeEq;
    if a.len() != b.len() {
        return false;
    }
    a.ct_eq(b).into()
}
