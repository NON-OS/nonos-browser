use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize)]
pub enum ConnectionStatus {
    Disconnected,
    Connecting,
    Bootstrapping,
    Connected,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum PrivacyMode {
    Fast,
    #[default]
    Balanced,
    Strong,
    Unsafe,
}

impl PrivacyMode {
    pub fn use_fastmode(&self) -> bool {
        matches!(self, PrivacyMode::Fast)
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum SelectedNetwork {
    #[default]
    Mainnet,
    Sepolia,
}

impl From<u8> for SelectedNetwork {
    fn from(v: u8) -> Self {
        match v {
            1 => SelectedNetwork::Sepolia,
            _ => SelectedNetwork::Mainnet,
        }
    }
}

impl From<SelectedNetwork> for u8 {
    fn from(n: SelectedNetwork) -> Self {
        match n {
            SelectedNetwork::Mainnet => 0,
            SelectedNetwork::Sepolia => 1,
        }
    }
}
