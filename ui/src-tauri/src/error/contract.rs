use std::fmt;

#[derive(Debug)]
pub enum ContractError {
    RpcFailed(String),
    TransactionFailed(String),
    InvalidAddress(String),
    EncodingError(String),
    Timeout,
}

impl fmt::Display for ContractError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RpcFailed(s) => write!(f, "RPC failed: {}", s),
            Self::TransactionFailed(s) => write!(f, "Transaction failed: {}", s),
            Self::InvalidAddress(s) => write!(f, "Invalid address: {}", s),
            Self::EncodingError(s) => write!(f, "Encoding error: {}", s),
            Self::Timeout => write!(f, "Request timeout"),
        }
    }
}

impl From<ContractError> for String {
    fn from(e: ContractError) -> String {
        e.to_string()
    }
}
