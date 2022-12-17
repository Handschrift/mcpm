use clap::Parser;

use mcpm::cli::{Action, Arguments, generate_application_files};
use mcpm::minecraft::init;
use mcpm::modrinth_wrapper::{download, search};

fn main() {
    let appdata_path = generate_application_files().expect("Couldn't create the data directory or the datafiles");
    let args: Arguments = Arguments::parse();

    match args.action {
        Action::Search { mod_name, limit } => {
            match search(mod_name, limit) {
                JsonError => {}
            };
        }
        Action::Install { mod_name } => {
            match download(mod_name) {
                JsonError => {}
                NetworkError => {}
                FileSystemError => {}
            };
        }
        Action::Init { minecraft_path } => {
            match init(appdata_path.as_path(), minecraft_path) {
                JsonError => {}
                NetworkError => {}
                FileSystemError => {}
            }
        }
        Action::Update {} => {}
    }
}
