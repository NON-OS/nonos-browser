#[derive(Debug, Default)]
pub struct BrowserState {
    pub next_tab_id: u32,
    pub history: Vec<String>,
}
