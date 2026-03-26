use bbean_api::state::AppState;
use bbean_core::EngineConfig;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("bbean=info".parse().unwrap())
                .add_directive("tower_http=debug".parse().unwrap()),
        )
        .init();

    let config = EngineConfig::default();
    let port = config.port;
    let host = config.host.clone();
    let state = AppState::new(config)?;

    println!();
    println!("  ========================================");
    println!("  BBEAN ENGINE v{}", bbean_core::VERSION);
    println!("  Browser-native DePIN for AI");
    println!("  ========================================");
    println!();
    println!("  API: http://{}:{}", host, port);
    println!();

    bbean_api::serve(&host, port, state).await
}
