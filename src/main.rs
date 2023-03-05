use clap::Parser;

use mcpm::cli::{Action, Arguments, generate_application_files};
use mcpm::minecraft::{init, MinecraftData, MinecraftDataEntry, MinecraftInstance};
use mcpm::modrinth_wrapper::{download, search};

fn main() {
    let args: Arguments = Arguments::parse();

    match args.action {
        Action::Search { mod_name, limit } => {
            match search(mod_name, limit) {
                JsonError => {}
            };
        }
        Action::Install { mod_name } => {
            match download(mod_name, current_minecraft_instance.get_minecraft_instance(&appdata_path).unwrap()) {
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
