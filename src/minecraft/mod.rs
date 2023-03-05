use std::{fs, io};

use regex::Regex;
use serde::Deserialize;
use serde::Serialize;

use crate::common::{McpmDataError};

#[derive(Deserialize, Serialize, Clone)]
pub struct MinecraftInstance {
    pub loader: String,
    pub minecraft_version: String,
    pub mods: Vec<InstalledMod>,
}

impl MinecraftInstance {
    pub fn new() -> MinecraftInstance {
        MinecraftInstance {
            loader: "".to_string(),
            minecraft_version: "".to_string(),
            mods: Vec::new(),
        }
    }

    pub fn create_mcpm_json(&self) -> Result<(), McpmDataError> {
        let json = serde_json::to_string(&self)?;
        fs::write("mcpm.lock", json)?;
        Ok(())
    }

    pub fn current() -> Result<MinecraftInstance, McpmDataError> {
        let json = fs::read_to_string("mcpm.lock")?;
        let data: MinecraftInstance = serde_json::from_str(&json)?;

        Ok(data)
    }

    pub fn add_mod(&mut self, minecraft_mod: InstalledMod) {
        self.mods.push(minecraft_mod)
    }

    pub fn save(&self) -> Result<(), McpmDataError> {
        let json = serde_json::to_string(&self)?;
        fs::write("mcpm.lock", json)?;
        Ok(())
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct InstalledMod {
    pub name: String,
    pub slug: String,
    pub version: String,
}


pub fn init() -> Result<(), McpmDataError> {
    let minecraft_version_regex = Regex::new(r"^1\.([0-9]{2}|[0-9])(?:\.([0-9]{2}|[0-9]))?$").unwrap();


    let mut loader = String::new();
    let mut version = String::new();
    println!("Please enter the modloader you are using (Forge|Fabric|Quilt):");
    io::stdin().read_line(&mut loader)?;
    while !loader.trim().to_lowercase().eq("forge") && !loader.trim().to_lowercase().eq("fabric") && !loader.trim().to_lowercase().eq("quilt") {
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

    new_minecraft_instance.loader = loader.trim().to_string();
    new_minecraft_instance.minecraft_version = version.trim().to_string();
    new_minecraft_instance.create_mcpm_json()?;

    Ok(())
}

