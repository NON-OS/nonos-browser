mod browser;
mod network;
mod node;
mod types;
mod wallet;

pub use browser::BrowserState;
pub use network::NetworkState;
pub use node::{NodeInfo, NodeState};
pub use types::{ConnectionStatus, SelectedNetwork};
pub use wallet::WalletState;

use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct AppState {
    pub network: Arc<RwLock<NetworkState>>,
    pub wallet: Arc<RwLock<WalletState>>,
    pub nodes: Arc<RwLock<NodeState>>,
    pub browser: Arc<RwLock<BrowserState>>,
    pub selected_network: AtomicU8,
}

impl AppState {
    pub fn get_selected_network(&self) -> SelectedNetwork {
        SelectedNetwork::from(self.selected_network.load(Ordering::Relaxed))
    }

    pub fn set_selected_network(&self, network: SelectedNetwork) {
        self.selected_network
            .store(network.into(), Ordering::Relaxed);
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            network: Arc::new(RwLock::new(NetworkState::default())),
            wallet: Arc::new(RwLock::new(WalletState::default())),
            nodes: Arc::new(RwLock::new(NodeState::default())),
            browser: Arc::new(RwLock::new(BrowserState::default())),
            selected_network: AtomicU8::new(0),
        }
    }
}
