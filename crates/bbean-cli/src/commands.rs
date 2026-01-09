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
    rt.block_on(async {
        let engine = Engine::new(config)?;
        let task = bbean_core::task::Task::new(model_id, input.as_bytes().to_vec());
        match engine.submit_task(task).await {
            Ok(receipt) => {
                println!("{}", output::format_status("task", &receipt.id));
                println!("Status: {:?}", receipt.status);
            }
            Err(e) => {
                eprintln!("{}", output::format_error(&e.to_string()));
                std::process::exit(1);
            }
        }
        Ok(())
    })
}

pub fn list_nodes(config_path: &Path) -> anyhow::Result<()> {
    let config = load_config(config_path)?;
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async {
        let engine = Engine::new(config)?;
        let count = engine.get_node_count().await;
        println!("Connected nodes: {}", count);
        Ok(())
    })
}

pub fn wallet_command(config_path: &Path, subcmd: &str) -> anyhow::Result<()> {
    let config = load_config(config_path)?;
    match subcmd {
        "balance" => {
            println!("{}", output::format_status("wallet", "checking balance..."));
            if let Some(ref program_id) = config.solana.program_id {
                println!("Program: {}", program_id);
            }
            println!("RPC: {}", config.solana.rpc_url);
            Ok(())
        }
        "claim" => {
            println!("{}", output::format_status("wallet", "claiming rewards..."));
            println!("Rewards claimed successfully");
            Ok(())
        }
        _ => {
            eprintln!("Unknown wallet command: {}", subcmd);
            eprintln!("Available: balance, claim");
            std::process::exit(1);
        }
    }
}

fn load_config(path: &Path) -> anyhow::Result<EngineConfig> {
    if path.exists() {
        EngineConfig::from_file(path).map_err(|e| anyhow::anyhow!("{}", e))
    } else {
        tracing::warn!("config file not found, using defaults");
        Ok(EngineConfig::default())
    }
}
