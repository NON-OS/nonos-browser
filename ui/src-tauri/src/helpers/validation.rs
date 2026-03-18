use crate::error::{ContractError, WalletError};

pub fn validate_password(password: &str) -> Result<(), WalletError> {
    if password.len() < 8 {
        return Err(WalletError::InvalidPassword);
    }
    Ok(())
}

pub fn validate_mnemonic(mnemonic: &str) -> Result<(), WalletError> {
    let words: Vec<&str> = mnemonic.split_whitespace().collect();
    if words.len() != 12 && words.len() != 24 {
        return Err(WalletError::InvalidMnemonic);
    }
    Ok(())
}

pub fn validate_eth_address(address: &str) -> Result<(), ContractError> {
    if !address.starts_with("0x") {
        return Err(ContractError::InvalidAddress("must start with 0x".into()));
    }
    if address.len() != 42 {
        return Err(ContractError::InvalidAddress(
            "must be 42 characters".into(),
        ));
    }
    let hex_part = &address[2..];
    if !hex_part.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(ContractError::InvalidAddress(
            "invalid hex characters".into(),
        ));
    }
    Ok(())
}

pub fn validate_amount(amount: &str) -> Result<f64, ContractError> {
    let parsed: f64 = amount
        .parse()
        .map_err(|_| ContractError::EncodingError("invalid amount format".into()))?;
    if parsed <= 0.0 {
        return Err(ContractError::EncodingError(
            "amount must be positive".into(),
        ));
    }
    Ok(parsed)
}

pub fn validate_tier(tier: u8) -> Result<(), ContractError> {
    if tier > 4 {
        return Err(ContractError::EncodingError("tier must be 0-4".into()));
    }
    Ok(())
}
