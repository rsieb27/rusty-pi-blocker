mod cli;
mod dns_server;
mod blocklist;
mod logger;
mod error;
mod server_control;
mod analysis;

use clap::Parser;
use cli::{Cli, Commands};
use color_eyre::eyre::Result;

fn main() -> Result<()> {
    let cli_args = Cli::parse();

    match cli_args.command {
        Commands::Start => {
            let dns_adblocker = dns_server::DnsAdBlocker::new()?;
            dns_adblocker.run()?;
        },
        Commands::Stop => server_control::stop_dns_server()?,
        Commands::Add { domain } => blocklist::add_domain(&domain)?, //add to blocklist
        Commands::Remove { domain } => blocklist::remove_domain(&domain)?, //remove from blocklist
        Commands::List => blocklist::list_blocked_domains("blocked_domains.txt")?, //print blocklist
        Commands::Analyze => {
            let analyzer = analysis::LogAnalyzer::new("blocked_log.csv");
            analyzer.analyze()?;
        }
    };

    Ok(())
}