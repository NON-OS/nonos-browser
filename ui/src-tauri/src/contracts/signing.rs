use super::rlp;
use k256::ecdsa::SigningKey;
use tiny_keccak::{Hasher, Keccak};

pub fn sign_transaction(
    private_key_hex: &str,
    to: &str,
    value: u128,
    data: &[u8],
    nonce: u64,
    gas_limit: u64,
    gas_price: u128,
    chain_id: u64,
) -> Result<String, String> {
    let key_bytes = hex::decode(private_key_hex.trim_start_matches("0x"))
        .map_err(|e| format!("Invalid private key: {}", e))?;
    let signing_key =
        SigningKey::from_slice(&key_bytes).map_err(|e| format!("Invalid key: {}", e))?;

    let to_bytes = hex::decode(to.trim_start_matches("0x"))
        .map_err(|e| format!("Invalid to address: {}", e))?;

    let mut items: Vec<Vec<u8>> = vec![
        rlp::encode_uint(nonce as u128),
        rlp::encode_uint(gas_price),
        rlp::encode_uint(gas_limit as u128),
        to_bytes.clone(),
        rlp::encode_uint(value),
        data.to_vec(),
        rlp::encode_uint(chain_id as u128),
        vec![],
        vec![],
    ];

    let unsigned_tx = rlp::encode_list(&items);
    let mut hasher = Keccak::v256();
    let mut hash = [0u8; 32];
    hasher.update(&unsigned_tx);
    hasher.finalize(&mut hash);

    let (signature, recovery_id) = signing_key
        .sign_prehash_recoverable(&hash)
        .map_err(|e| format!("Signing failed: {}", e))?;

    let sig_bytes = signature.to_bytes();
    let v = chain_id * 2 + 35 + recovery_id.to_byte() as u64;

    items[6] = rlp::encode_uint(v as u128);
    items[7] = sig_bytes[..32].to_vec();
    items[8] = sig_bytes[32..].to_vec();

    let signed_tx = rlp::encode_list(&items);
    Ok(format!("0x{}", hex::encode(signed_tx)))
}

pub fn get_address_from_private_key(private_key: &str) -> Result<String, String> {
    let key_bytes = hex::decode(private_key.trim_start_matches("0x"))
        .map_err(|e| format!("Invalid private key: {}", e))?;
    let signing_key =
        SigningKey::from_slice(&key_bytes).map_err(|e| format!("Invalid key: {}", e))?;
    let verifying_key = signing_key.verifying_key();
    let public_key = verifying_key.to_encoded_point(false);
    let public_key_bytes = &public_key.as_bytes()[1..];

    let mut hasher = Keccak::v256();
    let mut address_hash = [0u8; 32];
    hasher.update(public_key_bytes);
    hasher.finalize(&mut address_hash);
    Ok(format!("0x{}", hex::encode(&address_hash[12..])))
}
