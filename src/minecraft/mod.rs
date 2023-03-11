use std::{fs, io};
use std::io::ErrorKind;
use std::process::exit;

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

    pub fn remove_mod(&mut self, minecraft_mod: &InstalledMod){
        self.mods.retain(|x| {x.slug != minecraft_mod.slug});
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

pub fn uninstall(mod_name: String) -> Result<(), McpmDataError>{
    let mut current_instance = MinecraftInstance::current()?;
    let current_mod = current_instance.mods.iter().find(|minecraft_mod| {minecraft_mod.slug == mod_name}).cloned();
    if current_mod.is_some() {
        let current_mod = current_mod.unwrap();
        println!("Removing file...");
        match fs::remove_file(format!("mods/{}",&current_mod.version )){
            Err(error) => {
                match error.kind() {
                    ErrorKind::NotFound => {
                        println!("File not found... removing entry from lockfile.")
                    },
                    ErrorKind::PermissionDenied => {
                        println!("Couldn't delete file permission denied... exiting.");
                        exit(1);
                    },
                    _ => {}
                }
            },
            _ok => {}
        };
        println!("Updating lockfile...");
        current_instance.remove_mod(&current_mod);
        current_instance.save()?;
        println!("The mod {} has been successfully removed.", current_mod.name);
    } else {
        println!("This mod isn't installed.")
    }
    Ok(())
}