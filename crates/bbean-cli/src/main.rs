use std::path::PathBuf;

mod commands;
mod output;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("bbean=info".parse().unwrap()),
        )
        .init();

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        print_usage();
        return Ok(());
    }

    let config_path = PathBuf::from(
        std::env::var("BBEAN_CONFIG").unwrap_or_else(|_| "config.json".into()),
    );

    match args[1].as_str() {
        "start" => commands::start_node(&config_path),
        "status" => commands::show_status(&config_path),
        "submit" => {
            let model = args.get(2).map(|s| s.as_str()).unwrap_or("default");
            let input = args.get(3).map(|s| s.as_str()).unwrap_or("");
            commands::submit_task(&config_path, model, input)
        }
        "nodes" => commands::list_nodes(&config_path),
        "wallet" => {
            let subcmd = args.get(2).map(|s| s.as_str()).unwrap_or("balance");
            commands::wallet_command(&config_path, subcmd)
        }
        "version" => {
            println!("bbean-cli v{}", bbean_core::VERSION);
            Ok(())
        }
        "help" | "--help" | "-h" => {
            print_usage();
            Ok(())
        }
        other => {
            eprintln!("unknown command: {}", other);
            print_usage();
            std::process::exit(1);
        }
    }
}

fn print_usage() {
    println!("bbean-cli v{}", bbean_core::VERSION);
    println!();
    println!("Usage: bbean <command> [options]");
    println!();
    println!("Commands:");
    println!("  start           Start a BBEAN compute node");
    println!("  status          Show engine status");
    println!("  submit <model>  Submit an inference task");
    println!("  nodes           List connected nodes");
    println!("  wallet <cmd>    Wallet operations (balance, claim)");
    println!("  version         Show version");
    println!("  help            Show this help message");
    println!();
    println!("Environment:");
    println!("  BBEAN_CONFIG    Path to config file (default: config.json)");
}
