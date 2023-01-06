use std::error::Error;
use std::fs;
use std::path::Path;

use serde::Deserialize;
use serde::Serialize;

use crate::common::Mod;

#[derive(Deserialize, Serialize)]
pub struct MinecraftData {
    pub instances: Vec<MinecraftInstance>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct MinecraftInstance {
    pub default: bool,
    pub path: String,
    pub loader: String,
    pub minecraft_version: String,
}

#[derive(Deserialize, Serialize)]
pub struct MinecraftModList {
    pub mods: Vec<Mod>,
}

impl MinecraftModList {
    pub fn new() -> MinecraftModList {
        MinecraftModList {
            mods: Vec::new(),
        }
    }
}

impl MinecraftInstance {
    pub fn new() -> MinecraftInstance {
        MinecraftInstance {
            default: false,
            path: "".to_string(),
            loader: "".to_string(),
            minecraft_version: "".to_string(),
        }
    }
    pub fn create_mcpm_json(&self) -> Result<(), McpmDataError> {
        let json = serde_json::to_string(&MinecraftModList::new())?;
        let path = Path::new(&self.path).join("mcpm.json");
        fs::write(path.as_path(), json)?;
        Ok(())
    }
}

impl MinecraftData {
    pub fn new() -> MinecraftData {
        MinecraftData {
            instances: Vec::new()
        }
    }

    pub fn parse_existing(path: &Path) -> Result<MinecraftData, McpmDataError> {
        let json = fs::read_to_string(path)?;
        let data: MinecraftData = serde_json::from_str::<MinecraftData>(&json)?;

        Ok(data)
    }

    fn save(&self, data_path: &Path) -> Result<(), McpmDataError> {
        let json = serde_json::to_string(&self)?;
        fs::write(data_path, json)?;
        Ok(())
    }

    fn add_minecraft_instance(&mut self, instance: MinecraftInstance) {
        self.instances.push(instance)
    }
}


pub fn init(data_path: &Path, minecraft_path: String) -> Result<(), McpmDataError> {
    let mut data = MinecraftData::parse_existing(data_path)?;
    let minecraft_version_regex = Regex::new(r"^1\.([0-9]{2}|[0-9])(?:\.([0-9]{2}|[0-9]))?$").unwrap();

    if data.instances.iter().any(|x| {
        x.path == minecraft_path
    }) {
        println!("This is already an existing instance!");
        return Ok(());
    }

    let mut loader = String::new();
    let mut version = String::new();
    println!("Please enter the modloader you are using (Forge|Fabric|Quilt):");
    io::stdin().read_line(&mut loader)?;
    while !loader.to_lowercase().eq("forge") || !loader.to_lowercase().eq("fabric") || !loader.to_lowercase().eq("quilt") {
        println!("You didn't enter a valid modloader! Please enter Forge, Fabric or Quilt: ");
        io::stdin().read_line(&mut loader)?;
    }

    println!("Please enter your minecraft version: ");
    io::stdin().read_line(&mut version)?;
    while minecraft_version_regex.is_match(version.as_str()) {
        println!("This is not a valid minecraft version, please type a valid one: ");
        io::stdin().read_line(&mut version)?;

    }

    let mut new_minecraft_instance = MinecraftInstance::new();
    new_minecraft_instance.path = minecraft_path;

    if data.instances.is_empty() {
        new_minecraft_instance.default = true;
    }

    new_minecraft_instance.loader = loader.trim().to_string();
    new_minecraft_instance.minecraft_version = version.trim().to_string();
    new_minecraft_instance.create_mcpm_json()?;
    data.add_minecraft_instance(new_minecraft_instance);
    data.save(data_path)?;

    Ok(())
}

