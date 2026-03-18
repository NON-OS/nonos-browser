use super::{config::Network, rpc, signing};

pub async fn send_on_network(
    network: Network,
    private_key: &str,
    to: &str,
    value: u128,
    data: Vec<u8>,
    gas_limit: u64,
) -> Result<String, String> {
    let from = signing::get_address_from_private_key(private_key)?;
    let nonce = rpc::get_nonce(network, &from).await?;
    let gas_price = rpc::get_gas_price(network).await?;

    let signed_tx = signing::sign_transaction(
        private_key,
        to,
        value,
        &data,
        nonce,
        gas_limit,
        gas_price,
        network.chain_id(),
    )?;

    rpc::send_raw_transaction(network, &signed_tx).await
}

pub async fn send_mainnet(
    private_key: &str,
    to: &str,
    value: u128,
    data: Vec<u8>,
) -> Result<String, String> {
    send_on_network(Network::Mainnet, private_key, to, value, data, 150000).await
}

pub async fn send_sepolia(
    private_key: &str,
    to: &str,
    value: u128,
    data: Vec<u8>,
    gas_limit: u64,
) -> Result<String, String> {
    send_on_network(Network::Sepolia, private_key, to, value, data, gas_limit).await
}
