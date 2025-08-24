//! Rosetta Ruchy MCP Server
//!
//! A Model Context Protocol (MCP) server that provides real-time code translation
//! capabilities to Claude Code agents, allowing seamless conversion from any
//! supported language to Ruchy with immediate formal verification and performance analysis.

use anyhow::Result;
use clap::{Arg, Command};
use rosetta_ruchy_mcp::MCPServer;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let matches = Command::new("rosetta-ruchy-mcp")
        .version(env!("CARGO_PKG_VERSION"))
        .about("MCP server for code translation to Ruchy with formal verification")
        .arg(
            Arg::new("host")
                .long("host")
                .value_name("HOST")
                .default_value("127.0.0.1")
                .help("Host to bind the server to"),
        )
        .arg(
            Arg::new("port")
                .long("port")
                .value_name("PORT")
                .default_value("8080")
                .help("Port to bind the server to"),
        )
        .arg(
            Arg::new("ruchy-path")
                .long("ruchy-path")
                .value_name("PATH")
                .default_value("ruchy")
                .help("Path to the ruchy compiler executable"),
        )
        .get_matches();

    let host = matches.get_one::<String>("host").unwrap();
    let port = matches.get_one::<String>("port").unwrap().parse::<u16>()?;
    let ruchy_path = matches.get_one::<String>("ruchy-path").unwrap().to_string();

    info!("Starting Rosetta Ruchy MCP Server on {}:{}", host, port);
    info!("Using Ruchy compiler at: {}", ruchy_path);

    let server = MCPServer::new(host.to_string(), port, ruchy_path);
    server.start().await?;

    Ok(())
}
