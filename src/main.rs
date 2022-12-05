use clap::Parser;

use mcpm::cli::{Action, Arguments, check_application_files};
use mcpm::modrinth_wrapper::search;

fn main() {
    //https://github.com/yashs662/rust_kanban/tree/main/src
    check_application_files().expect("Couldn't create the data directory or the datafiles");
    let args: Arguments = Arguments::parse();

    match args.action {
        Action::Search { mod_name, limit } => {
            search(mod_name, limit).expect("failed to search");
        }
        Action::Install { mod_name } => {}
        _ => ()
    }
}
