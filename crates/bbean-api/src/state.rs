use std::sync::Arc;
use tokio::sync::RwLock;

use bbean_core::{Engine, EngineConfig};

pub struct AppState {
    pub engine: RwLock<Engine>,
    pub config: EngineConfig,
    pub start_time: chrono::DateTime<chrono::Utc>,
}

impl AppState {
    pub fn new(config: EngineConfig) -> anyhow::Result<Arc<Self>> {
        let engine = Engine::new(config.clone())
            .map_err(|e| anyhow::anyhow!("{}", e))?;
        Ok(Arc::new(Self {
            engine: RwLock::new(engine),
            config,
            start_time: chrono::Utc::now(),
        }))
    }

    pub fn uptime_secs(&self) -> i64 {
        (chrono::Utc::now() - self.start_time).num_seconds()
    }
}
