mod cli;
mod dns_server;
mod blocklist;
mod logger;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let cli_args = Cli::parse();

    match cli_args.command {
        Commands::Start => dns_server::start_dns_server(),
        Commands::Add { domain } => blocklist::add_domain(&domain),
        Commands::Remove { domain } => blocklist::remove_domain(&domain),
        Commands::List => blocklist::list_blocked_domains(),
    }
}