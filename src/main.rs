use std::path::PathBuf;
use std::process::exit;

use clap::Parser;


use mcpm::cli::{Action, Arguments};
use mcpm::minecraft::{init, MinecraftInstance, uninstall};
use mcpm::modrinth_wrapper::{download, search};

pub const LOCK_FILE_NAME: &str = "mcpm.lock";

#[tokio::main]
async fn main() {
    let args: Arguments = Arguments::parse();

    match args.action {
        Action::Search { mod_name, limit } => {
            search(mod_name, limit).await.expect("Searching a mod failed. Please try again later or check your internet connection");
        }
        Action::Init {} => {
            init().expect("There was an error initialising and creating the mcpm.lock");
        }
        Action::Install { mod_name } => {
            handle_missing_lock_file();
            download(mod_name, MinecraftInstance::current().unwrap()).await.expect("Error downloading the file");
        }
        Action::Remove {mod_name} => {
            uninstall(mod_name).expect("There was an error uninstalling the mod.");
        }
        _ => {}
    }
}

fn handle_missing_lock_file() {
    if !PathBuf::from(LOCK_FILE_NAME).exists() {
        eprintln!("No lock file found in the current directory... closing application");
        eprintln!("Please enter your minecraft folder and run mcpm init!");
        exit(1);
    }
}