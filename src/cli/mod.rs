use std::borrow::Borrow;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use clap::Parser;
use clap::Subcommand;
use directories::BaseDirs;

use crate::minecraft::MinecraftData;

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
        #[clap(long, short, default_value="20")]
        limit: u16,
    },
    Install {
        mod_name: String,
    },
    Init {
        minecraft_path: String,
    },
    Update {},
}


pub fn generate_application_files() -> Result<PathBuf, Box<dyn Error>> {
    let base_dir = BaseDirs::new();

    let dir = match &base_dir {
        None => Path::new("."),
        Some(dir) => {
            dir.data_local_dir()
        }
    };

    let dir = dir.join(Path::new("mcpcm/data.json"));
    let data = MinecraftData { path: "test".to_string() };

    if !dir.exists() {
        fs::create_dir(dir.as_path().parent().unwrap())?;

        let mut file = File::create(dir.as_path())?;

        let json = serde_json::to_string(&data)?;

        file.write(json.as_bytes())?;

        println!("Seems like you started mcpm your first time... creating local files");
    }

    Ok(dir)
}