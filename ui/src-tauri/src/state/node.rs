use serde::Serialize;
use std::sync::atomic::AtomicU64;

#[derive(Debug, Clone, Serialize)]
pub struct NodeInfo {
    pub id: String,
    pub address: String,
    pub quality_score: f64,
    pub latency_ms: u32,
    pub connected: bool,
}

#[derive(Debug, Default)]
pub struct NodeState {
    pub nodes: Vec<NodeInfo>,
    pub embedded_running: bool,
    pub embedded_quality: f64,
    pub total_requests: AtomicU64,
    pub embedded_pid: Option<u32>,
    pub node_id: Option<String>,
    pub api_addr: String,
    pub p2p_port: u16,
}
