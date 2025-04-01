use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "RUST DNS AD BLOCKER <3")]
#[command(about = "My attempt at an ad blocker written in mostly Rust")]

pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    //start dns server
    Start,
    //add domain to the blocklist
    Add { domain: String },
    //remove a domain from the blocklist
    Remove { domain: String },
    //list all the blocked domains
    List,
}