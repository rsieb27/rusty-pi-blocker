mod cli;
mod dns_server;
mod blocklist;
mod logger;
mod error;

use clap::Parser;
use cli::{Cli, Commands};
use color_eyre::eyre::{Result, eyre};

fn main() -> Result<()> {
    let cli_args = Cli::parse();

    match cli_args.command {
        Commands::Start => dns_server::start_dns_server(), //run dns server
        Commands::Add { domain } => blocklist::add_domain(&domain), //add to blocklist
        Commands::Remove { domain } => blocklist::remove_domain(&domain), //remove from blocklist
        Commands::List => blocklist::list_blocked_domains(), //print blocklist
    }?;

    Ok(())
}