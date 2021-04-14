mod cli;
mod config;
mod server;

use anyhow::{Context, Result};
use std::convert::TryFrom;
use structopt::StructOpt;

use crate::config::file::ConfigFile;
use crate::config::Config;
use crate::server::Server;

fn resolve_config(cli_arguments: cli::Cli) -> Result<Config> {
    if let Some(config_path) = cli_arguments.config {
        let config_file = ConfigFile::from_file(config_path)?;
        let config = Config::try_from(config_file)?;

        return Ok(config);
    }

    // Otherwise configuration is build from CLI arguments
    Config::try_from(cli_arguments).with_context(|| anyhow::Error::msg("OK"))
}

pub async fn run() -> Result<()> {
    let cli_arguments = cli::Cli::from_args();
    let config = resolve_config(cli_arguments)?;
    let server = Server::new(config);

    server.run().await;

    Ok(())
}
