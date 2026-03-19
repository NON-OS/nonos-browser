#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod browser;
mod contracts;
mod error;
mod helpers;
mod lp_staking;
mod network;
mod node;
mod privacy;
mod proxy;
mod staking;
mod state;
mod types;
mod wallet;
mod work_metrics;

use state::AppState;
use tauri::Manager;

#[tauri::command]
fn get_app_info() -> types::AppInfo {
    types::AppInfo {
        name: "NONOS Ecosystem",
        version: env!("CARGO_PKG_VERSION"),
        platform: std::env::consts::OS,
        arch: std::env::consts::ARCH,
        build: if cfg!(debug_assertions) {
            "debug"
        } else {
            "release"
        },
    }
}

fn main() {
    let state = AppState::default();
    let network_state_for_setup = state.network.clone();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(state)
        .setup(move |app| {
            let window = app.get_webview_window("main").unwrap();

            tauri::async_runtime::spawn(async move {
                proxy::start_local_proxy_server().await;
            });

            let network_for_spawn = network_state_for_setup.clone();
            tauri::async_runtime::spawn(async move {
                let _ = network::auto_start_nym(network_for_spawn).await;
            });

            window
                .eval(
                    r#"
                const invoke = window.__TAURI__.core.invoke;
                const listen = window.__TAURI__.event.listen;
                window.nonos = {
                    version: '1.0.5',
                    network: {
                        connect: () => invoke('network_connect'),
                        disconnect: () => invoke('network_disconnect'),
                        getStatus: () => invoke('network_get_status'),
                        newIdentity: () => invoke('network_new_identity'),
                    },
                    wallet: {
                        getStatus: () => invoke('wallet_get_status'),
                        create: (password) => invoke('wallet_create', { password }),
                        import: (mnemonic, password) => invoke('wallet_import', { mnemonic, password }),
                        unlock: (password) => invoke('wallet_unlock', { password }),
                        lock: () => invoke('wallet_lock'),
                        getAddress: () => invoke('wallet_get_address'),
                        sendEth: (to, amount) => invoke('wallet_send_eth', { to, amount: String(amount) }),
                        sendNox: (to, amount) => invoke('wallet_send_nox', { to, amount: String(amount) }),
                        getTransactions: () => invoke('wallet_get_transactions'),
                        exists: () => invoke('wallet_check_exists'),
                        getStealthAddress: () => invoke('wallet_get_stealth_address'),
                        changePassword: (oldPassword, newPassword) => invoke('wallet_change_password', { oldPassword, newPassword }),
                        getSelectedNetwork: () => invoke('get_selected_network'),
                        setSelectedNetwork: (network) => invoke('set_selected_network', { network }),
                    },
                    staking: {
                        getStatus: () => invoke('staking_get_status'),
                        stake: (amount) => invoke('staking_stake', { amount }),
                        unstake: (amount) => invoke('staking_unstake', { amount }),
                        claimRewards: () => invoke('staking_claim_rewards'),
                        withdraw: () => invoke('staking_withdraw'),
                    },
                    lpStaking: {
                        getStatus: () => invoke('lp_get_status'),
                        getTiers: () => invoke('lp_get_tiers'),
                        lock: (amount, tier) => invoke('lp_lock', { amount: String(amount), tier }),
                        unlock: (lockId) => invoke('lp_unlock', { lockId }),
                        earlyUnlock: (lockId) => invoke('lp_early_unlock', { lockId }),
                        extendLock: (lockId, newTier) => invoke('lp_extend_lock', { lockId, newTier }),
                        claimRewards: (lockId) => invoke('lp_claim_rewards', { lockId }),
                        claimAllRewards: () => invoke('lp_claim_all_rewards'),
                        compoundRewards: (lockId) => invoke('lp_compound_rewards', { lockId }),
                    },
                    work: {
                        getMetrics: () => invoke('work_get_metrics'),
                        getDashboard: () => invoke('work_get_dashboard'),
                        getEpoch: () => invoke('work_get_epoch'),
                    },
                    node: {
                        getStatus: () => invoke('node_get_status'),
                        startEmbedded: () => invoke('node_start_embedded'),
                        stopEmbedded: () => invoke('node_stop_embedded'),
                        getConnected: () => invoke('node_get_connected'),
                    },
                    browser: {
                        navigate: (url) => invoke('browser_navigate', { url }),
                        getSocksProxy: () => invoke('browser_get_socks_proxy'),
                        proxyFetch: (url, options = {}) => invoke('proxy_fetch', {
                            url,
                            method: options.method || 'GET',
                            headers: options.headers || null,
                            body: options.body || null,
                        }),
                    },
                    privacy: {
                        getStats: () => invoke('privacy_get_stats'),
                        checkTracking: (domain) => invoke('privacy_check_tracking', { domain }),
                        blockDomain: (domain) => invoke('privacy_block_domain', { domain }),
                        generateIdentity: (name) => invoke('privacy_generate_identity', { name }),
                        getIdentityRoot: () => invoke('privacy_get_identity_root'),
                        cacheStore: (content) => invoke('privacy_cache_store', { content }),
                    },
                    getAppInfo: () => invoke('get_app_info'),
                    getContractAddresses: () => invoke('get_contract_addresses'),
                    onNetworkStatus: (callback) => {
                        return listen('nonos://network-status', (event) => callback(event.payload));
                    },
                    onIdentityChanged: (callback) => {
                        return listen('nonos://identity-changed', callback);
                    },
                    onNodeStarted: (callback) => {
                        return listen('nonos://node-started', callback);
                    },
                    onNodeStopped: (callback) => {
                        return listen('nonos://node-stopped', callback);
                    },
                };
            "#,
                )
                .ok();

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_app_info,
            contracts::commands::get_contract_addresses,
            network::commands::network_connect,
            network::commands::network_get_privacy_mode,
            network::commands::network_set_privacy_mode,
            network::disconnect::network_disconnect,
            network::disconnect::network_get_status,
            network::disconnect::network_new_identity,
            wallet::status::wallet_get_status,
            wallet::create::wallet_create,
            wallet::create::wallet_import,
            wallet::auth::wallet_unlock,
            wallet::auth::wallet_lock,
            wallet::query::wallet_get_address,
            wallet::send_eth::wallet_send_eth,
            wallet::send_nox::wallet_send_nox,
            wallet::query::wallet_get_transactions,
            wallet::query::wallet_check_exists,
            wallet::query::wallet_get_stealth_address,
            wallet::auth::wallet_change_password,
            wallet::network_select::get_selected_network,
            wallet::network_select::set_selected_network,
            staking::status::staking_get_status,
            staking::commands::staking_stake,
            staking::commands::staking_unstake,
            staking::commands::staking_claim_rewards,
            staking::commands::staking_withdraw,
            lp_staking::status::lp_get_status,
            lp_staking::status::lp_get_tiers,
            lp_staking::lock::lp_lock,
            lp_staking::lock::lp_unlock,
            lp_staking::lock::lp_early_unlock,
            lp_staking::lock::lp_extend_lock,
            lp_staking::rewards::lp_claim_rewards,
            lp_staking::rewards::lp_claim_all_rewards,
            lp_staking::rewards::lp_compound_rewards,
            work_metrics::commands::work_get_metrics,
            work_metrics::commands::work_get_dashboard,
            work_metrics::commands::work_get_epoch,
            node::status::node_get_status,
            node::lifecycle::node_start_embedded,
            node::lifecycle::node_stop_embedded,
            node::peers::node_get_connected,
            browser::navigate::browser_navigate,
            browser::tabs::browser_close_tab,
            browser::tabs::browser_get_tabs,
            browser::tabs::browser_get_socks_proxy,
            browser::fetch::proxy_fetch,
            browser::tabs::get_proxy_url,
            privacy::stats::privacy_get_stats,
            privacy::tracking::privacy_check_tracking,
            privacy::tracking::privacy_block_domain,
            privacy::identity::privacy_generate_identity,
            privacy::identity::privacy_get_identity_root,
            privacy::cache::privacy_cache_store,
        ])
        .run(tauri::generate_context!())
        .expect("error while running NONOS Ecosystem browser");
}
