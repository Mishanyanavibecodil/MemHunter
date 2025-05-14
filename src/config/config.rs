use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub private_key: String,
    pub rpc_endpoint: String,
    pub websocket_endpoint: String,
    pub quote_mint: String,
    pub quote_amount: f64,
    pub commitment_level: String,
    pub use_snipe_list: bool,
    pub snipe_list_refresh_interval: u64,
    pub check_if_mint_is_renounced: bool,
    pub auto_sell: bool,
    pub max_sell_retries: u32,
    pub auto_sell_delay: u64,
    pub log_level: String,
    pub take_profit: u32,
    pub stop_loss: u32,
    pub birdeye_api_key: Option<String>,
    pub min_pool_size: u64,
    pub proxy_urls: Vec<String>,
    pub cpu_threads: usize,
    pub enable_gpu: bool,
}

impl Config {
    pub async fn load_config() -> Result<Self, Box<dyn std::error::Error>> {
        let mut settings = config::Settings::default();
        settings.merge(config::File::with_name("config.json"))?;
        settings.merge(config::Environment::default())?;
        
        let config: Config = settings.try_into()?;
        Ok(config)
    }
}