use super::types::{
    EntropyMetrics, MixerOpsMetrics, RegistryOpsMetrics, TrafficRelayMetrics, ZkProofMetrics,
};

pub fn calculate_traffic_score(m: &TrafficRelayMetrics) -> f64 {
    const BASELINE: u64 = 1_000_000_000;
    let ratio = m.bytes_relayed as f64 / BASELINE as f64;
    (ratio * 100.0).min(100.0)
}

pub fn calculate_zk_score(m: &ZkProofMetrics) -> f64 {
    const BASELINE: u64 = 1000;
    let total = m.proofs_generated + m.proofs_verified;
    let ratio = total as f64 / BASELINE as f64;
    (ratio * 100.0).min(100.0)
}

pub fn calculate_mixer_score(m: &MixerOpsMetrics) -> f64 {
    const BASELINE: u64 = 100;
    let total = m.deposits_processed + m.spends_processed;
    let ratio = total as f64 / BASELINE as f64;
    (ratio * 100.0).min(100.0)
}

pub fn calculate_entropy_score(m: &EntropyMetrics) -> f64 {
    const BASELINE: u64 = 10_000_000;
    let ratio = m.entropy_bytes_contributed as f64 / BASELINE as f64;
    (ratio * 100.0).min(100.0)
}

pub fn calculate_registry_score(m: &RegistryOpsMetrics) -> f64 {
    const BASELINE: u64 = 500;
    let total = m.registrations_processed + m.lookups_served;
    let ratio = total as f64 / BASELINE as f64;
    (ratio * 100.0).min(100.0)
}
