use rand::rngs::OsRng;
use rand::RngCore;
use std::sync::OnceLock;

static SESSION_TOKEN: OnceLock<String> = OnceLock::new();

fn get_token() -> &'static str {
    SESSION_TOKEN.get_or_init(|| {
        let mut bytes = [0u8; 32];
        OsRng.fill_bytes(&mut bytes);
        hex::encode(bytes)
    })
}

pub fn validate_session(token: &str) -> bool {
    get_token() == token
}
