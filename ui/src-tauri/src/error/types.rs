use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Wallet(WalletError),
    Contract(ContractError),
    Network(NetworkError),
    Proxy(ProxyError),
    Validation(String),
}

#[derive(Debug)]
pub enum WalletError {
    NotInitialized,
    Locked,
    InvalidPassword,
    InvalidMnemonic,
    InsufficientBalance { have: String, need: String },
    SigningFailed(String),
}

#[derive(Debug)]
pub enum ContractError {
    RpcFailed(String),
    TransactionFailed(String),
    InvalidAddress(String),
    EncodingError(String),
    Timeout,
}

#[derive(Debug)]
pub enum NetworkError {
    NotConnected,
    ConnectionFailed(String),
    BinaryNotFound,
    ProcessFailed(String),
    Timeout,
}

#[derive(Debug)]
pub enum ProxyError {
    FetchFailed(String),
    InvalidUrl(String),
    PrivateNetwork,
    TlsError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Wallet(e) => write!(f, "{}", e),
            Self::Contract(e) => write!(f, "{}", e),
            Self::Network(e) => write!(f, "{}", e),
            Self::Proxy(e) => write!(f, "{}", e),
            Self::Validation(s) => write!(f, "Validation: {}", s),
        }
    }
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

impl fmt::Display for NetworkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotConnected => write!(f, "Not connected"),
            Self::ConnectionFailed(s) => write!(f, "Connection failed: {}", s),
            Self::BinaryNotFound => write!(f, "Nym binary not found"),
            Self::ProcessFailed(s) => write!(f, "Process failed: {}", s),
            Self::Timeout => write!(f, "Connection timeout"),
        }
    }
}

impl fmt::Display for ProxyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FetchFailed(s) => write!(f, "Fetch failed: {}", s),
            Self::InvalidUrl(s) => write!(f, "Invalid URL: {}", s),
            Self::PrivateNetwork => write!(f, "Private network access blocked"),
            Self::TlsError(s) => write!(f, "TLS error: {}", s),
        }
    }
}

impl From<AppError> for String {
    fn from(e: AppError) -> String {
        e.to_string()
    }
}

impl From<WalletError> for String {
    fn from(e: WalletError) -> String {
        e.to_string()
    }
}

impl From<ContractError> for String {
    fn from(e: ContractError) -> String {
        e.to_string()
    }
}

impl From<NetworkError> for String {
    fn from(e: NetworkError) -> String {
        e.to_string()
    }
}

impl From<ProxyError> for String {
    fn from(e: ProxyError) -> String {
        e.to_string()
    }
}
