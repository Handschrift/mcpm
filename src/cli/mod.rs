use std::borrow::Borrow;
use std::error::Error;
use std::fs;
use clap::Parser;
use clap::Subcommand;

#[derive(Parser)]
#[clap(author = "Handschrift", version = "0.0.1", about = "Package manager for minecraft modloaders")]
pub struct Arguments {
    #[command(subcommand)]
    pub action: Action,
}

#[derive(Subcommand)]
pub enum Action {
    Search {
        mod_name: String,
        #[clap(long, short, default_value = "20")]
        limit: u16,
    },
    Install {
        mod_name: String,
    },
    Init {},
    Update {},
}