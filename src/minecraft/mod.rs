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

#[derive(Deserialize, Serialize)]
pub struct MinecraftInstance {
    pub default: bool,
    pub path: String,
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
    fn new() -> MinecraftInstance {
        MinecraftInstance {
            default: false,
            path: "".to_string(),
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

    if data.instances.iter().any(|x| {
        x.path == minecraft_path
    }) {
        println!("This is already an existing instance!");
        return Ok(())
    }

    let mut new_minecraft_instance = MinecraftInstance::new();
    new_minecraft_instance.path = minecraft_path;
    new_minecraft_instance.default = true;
    new_minecraft_instance.create_mcpm_json()?;
    data.add_minecraft_instance(new_minecraft_instance);
    data.save(data_path)?;
    
    Ok(())
}

