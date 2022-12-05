use clap::Parser;

use mcpm::cli::{Action, Arguments, generate_application_files};
use mcpm::minecraft::init;
use mcpm::modrinth_wrapper::search;

fn main() {
    let appdata_path = generate_application_files().expect("Couldn't create the data directory or the datafiles");
    let args: Arguments = Arguments::parse();

    match args.action {
        Action::Search { mod_name, limit } => {
            search(mod_name, limit).expect("failed to search");
        }
        Action::Install { mod_name } => {}
        Action::Init { minecraft_path } => {
            init(minecraft_path);
        }
        _ => ()
    }
}
