use std::path::PathBuf;
use std::process::exit;

use clap::Parser;

use mcpm::cli::{Action, Arguments};
use mcpm::minecraft::{init, MinecraftInstance};
use mcpm::modrinth_wrapper::{download, search};

pub const LOCK_FILE_NAME: &str = "mcpm.lock";

fn main() {
    let args: Arguments = Arguments::parse();

    match args.action {
        Action::Search { mod_name, limit } => {
            search(mod_name, limit).expect("Searching a mod failed. Please try again later or check your internet connection");
        }
        Action::Init {} => {
            init().expect("There was an error initialising and creating the mcpm.lock");
        }
        Action::Install { mod_name } => {
            handle_missing_lock_file();
            download(mod_name, MinecraftInstance::current().unwrap()).expect("There was an error downloading the file");
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