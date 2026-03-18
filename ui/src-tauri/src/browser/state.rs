use std::collections::HashMap;
use std::sync::Mutex;

static BROWSER_WINDOWS: std::sync::OnceLock<Mutex<HashMap<u32, String>>> =
    std::sync::OnceLock::new();

pub fn get_browser_windows() -> &'static Mutex<HashMap<u32, String>> {
    BROWSER_WINDOWS.get_or_init(|| Mutex::new(HashMap::new()))
}
