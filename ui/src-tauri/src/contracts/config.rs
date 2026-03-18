pub const CHAIN_ID_MAINNET: u64 = 1;
pub const CHAIN_ID_SEPOLIA: u64 = 11155111;

pub const RPC_ENDPOINTS_MAINNET: &[&str] = &[
    "https://ethereum.publicnode.com",
    "https://1rpc.io/eth",
    "https://eth.merkle.io",
    "https://rpc.payload.de",
];

pub const RPC_ENDPOINTS_SEPOLIA: &[&str] = &[
    "https://ethereum-sepolia-rpc.publicnode.com",
    "https://rpc.sepolia.org",
    "https://sepolia.drpc.org",
];

pub const NOX_TOKEN_ADDRESS_MAINNET: &str = "0x0a26c80Be4E060e688d7C23aDdB92cBb5D2C9eCA";
pub const NOX_TOKEN_ADDRESS_SEPOLIA: &str = "0xC87799c4517Dcdfc65bfefa3Be64Beb89668c66c";

pub const NOX_STAKING_POOL_SEPOLIA: &str = "0xb27DAe7EbE628ebe2E9302D7D2C71eF34Dd01705";
pub const PRIVACY_LIQUIDITY_POOL_SEPOLIA: &str = "0xb27DAe7EbE628ebe2E9302D7D2C71eF34Dd01705";

pub const PRIVACY_WORK_ECONOMY_SEPOLIA: &str = "0xAf8018e21Eff6F21BE305941f6d595Bd337c8bCA";
pub const NODE_OPERATOR_REWARDS_SEPOLIA: &str = "0x8cF7E025DDe69dA239392e54f5D344b434A62ba7";
pub const ZEROSTATE_STAKING_SEPOLIA: &str = "0xe6fD264976bcB896165525a8250908e824Fc9BD8";
pub const FEE_SWAP_SEPOLIA: &str = "0x98AE9CBE5D5C2Bee33F6Ba6771C8a6677E102dE6";
pub const ZEROSTATE_PASS_MAINNET: &str = "0x7b575DD8e8b111c52Ab1e872924d4Efd4DF403df";

pub const BALANCE_OF_SELECTOR: &str = "70a08231";
pub const SOCKS_PROXY: &str = "socks5h://127.0.0.1:1080";

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Network {
    Mainnet,
    Sepolia,
}

impl Network {
    pub fn rpc_endpoints(&self) -> &'static [&'static str] {
        match self {
            Network::Mainnet => RPC_ENDPOINTS_MAINNET,
            Network::Sepolia => RPC_ENDPOINTS_SEPOLIA,
        }
    }

    pub fn chain_id(&self) -> u64 {
        match self {
            Network::Mainnet => CHAIN_ID_MAINNET,
            Network::Sepolia => CHAIN_ID_SEPOLIA,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Network::Mainnet => "Ethereum Mainnet",
            Network::Sepolia => "Sepolia Testnet",
        }
    }
}
