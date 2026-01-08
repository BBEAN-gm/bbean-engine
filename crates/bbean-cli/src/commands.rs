use std::path::Path;

use bbean_core::{Engine, EngineConfig};

use crate::output;

pub fn start_node(config_path: &Path) -> anyhow::Result<()> {
    let config = load_config(config_path)?;
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async {
        let mut engine = Engine::new(config)?;
        engine.start().await?;
        println!("{}", output::format_status("engine", "running"));
        tokio::signal::ctrl_c().await?;
        engine.stop().await?;
        println!("{}", output::format_status("engine", "stopped"));
        Ok(())
    })
}

pub fn show_status(config_path: &Path) -> anyhow::Result<()> {
    let config = load_config(config_path)?;
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async {
        let engine = Engine::new(config)?;
        let node_count = engine.get_node_count().await;
        let running = engine.is_running();
        let rows = vec![
            ("Status", if running { "running" } else { "stopped" }),
            ("Version", bbean_core::VERSION),
        ];
        println!("{}", output::format_table("Engine Status", &rows));
        println!("Connected nodes: {}", node_count);
        Ok(())
    })
}

pub fn submit_task(config_path: &Path, model_id: &str, input: &str) -> anyhow::Result<()> {
    let config = load_config(config_path)?;
    let rt = tokio::runtime::Runtime::new()?;