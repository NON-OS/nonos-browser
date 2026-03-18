use serde::{Deserialize, Serialize};

pub const DAEMON_API_URL: &str = "http://127.0.0.1:8420";

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TrafficRelayMetrics {
    pub bytes_relayed: u64,
    pub relay_sessions: u64,
    pub successful_relays: u64,
    pub failed_relays: u64,
    pub avg_latency_ms: f64,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ZkProofMetrics {
    pub proofs_generated: u64,
    pub proofs_verified: u64,
    pub avg_generation_time_ms: f64,
    pub verification_failures: u64,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct MixerOpsMetrics {
    pub deposits_processed: u64,
    pub spends_processed: u64,
    pub total_value_mixed: u128,
    pub pool_participations: u64,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct EntropyMetrics {
    pub entropy_bytes_contributed: u64,
    pub entropy_requests_served: u64,
    pub quality_score: f64,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct RegistryOpsMetrics {
    pub registrations_processed: u64,
    pub lookups_served: u64,
    pub sync_operations: u64,
    pub failed_operations: u64,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct EpochInfo {
    pub current_epoch: u64,
    pub epoch_start_timestamp: u64,
    pub epoch_end_timestamp: u64,
    pub submitted_to_oracle: bool,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct WorkMetrics {
    pub traffic_relay: TrafficRelayMetrics,
    pub zk_proofs: ZkProofMetrics,
    pub mixer_ops: MixerOpsMetrics,
    pub entropy: EntropyMetrics,
    pub registry_ops: RegistryOpsMetrics,
    pub epoch: EpochInfo,
    pub total_work_score: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorkMetricsResponse {
    pub success: bool,
    pub data: WorkMetrics,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorkCategoryBreakdown {
    pub name: String,
    pub weight: u8,
    pub score: f64,
    pub raw_value: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorkDashboard {
    pub metrics: WorkMetrics,
    pub categories: Vec<WorkCategoryBreakdown>,
    pub estimated_epoch_reward: String,
    pub network_rank: Option<u32>,
    pub network_total_nodes: u32,
}
