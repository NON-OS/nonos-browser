pub fn format_wei(wei: u128) -> String {
    let eth = wei as f64 / 1e18;
    if eth >= 1.0 {
        format!("{:.4}", eth)
    } else if eth >= 0.0001 {
        format!("{:.6}", eth)
    } else {
        format!("{:.8}", eth)
    }
}
