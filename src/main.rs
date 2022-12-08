use clap::Parser;

use mcpm::cli::{Action, Arguments, generate_application_files};
use mcpm::minecraft::init;
use mcpm::modrinth_wrapper::{download, search};

fn main() {
    let appdata_path = generate_application_files().expect("Couldn't create the data directory or the datafiles");
    let args: Arguments = Arguments::parse();

    match args.action {
        Action::Search { mod_name, limit } => {
            search(mod_name, limit).expect("failed to search");
        }
        Action::Install { mod_name } => {
            download(mod_name);
        }
        Action::Init { minecraft_path } => {
            init(appdata_path.as_path(), minecraft_path).expect("Failed to initialize the minecraft instance");
        }
        _ => ()
    }
}
