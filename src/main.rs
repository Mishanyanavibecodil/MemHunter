mod config;
mod core;
mod services;
mod storage;
mod utils;

use std::sync::Arc;
use tokio::runtime::Builder;

fn main() {
    // Инициализация runtime
    let rt = Builder::multi_thread()
        .enable_all()
        .build()
        .expect("Failed to build Tokio runtime");

    rt.block_on(async move {
        // Загрузка конфигурации
        let config = config::load_config().await.expect("Failed to load config");
        
        // Создание общих ресурсов
        let logger = utils::logger::init_logger(&config.log_level);
        let proxy_manager = services::proxy::ProxyManager::new(config.proxy_urls);
        
        // Запуск основных сервисов
        let cpu_manager = core::cpu_manager::CpuManager::new(config.cpu_threads);
        let gpu_manager = core::gpu_manager::GpuManager::new();
        let ml_analyzer = core::ml_analyzer::MLCoinAnalyzer::new();
        
        // Запуск приложения
        pumpfun_trader::start(
            Arc::new(cpu_manager),
            Arc::new(gpu_manager),
            Arc::new(ml_analyzer),
            Arc::new(proxy_manager),
            config.clone(),
            logger,
        )
        .await;
    });
}

#[tokio::main]
async fn start(
    cpu_manager: Arc<CpuManager>,
    gpu_manager: Arc<GpuManager>,
    ml_analyzer: Arc<MLCoinAnalyzer>,
    proxy_manager: Arc<ProxyManager>,
    config: Config,
    logger: Logger,
) -> Result<(), Error> {
    // Реализация логики запуска приложения
    Ok(())
}