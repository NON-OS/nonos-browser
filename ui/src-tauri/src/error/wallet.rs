use std::fmt;

#[derive(Debug)]
pub enum WalletError {
    NotInitialized,
    Locked,
    InvalidPassword,
    InvalidMnemonic,
    InsufficientBalance { have: String, need: String },
    SigningFailed(String),
}

impl fmt::Display for WalletError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotInitialized => write!(f, "Wallet not initialized"),
            Self::Locked => write!(f, "Wallet is locked"),
            Self::InvalidPassword => write!(f, "Invalid password"),
            Self::InvalidMnemonic => write!(f, "Invalid mnemonic phrase"),
            Self::InsufficientBalance { have, need } => {
                write!(f, "Insufficient balance: have {}, need {}", have, need)
            }
            Self::SigningFailed(s) => write!(f, "Signing failed: {}", s),
        }
    }
}

impl From<WalletError> for String {
    fn from(e: WalletError) -> String {
        e.to_string()
    }
}
