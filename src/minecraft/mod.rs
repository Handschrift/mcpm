use std::{fs, io};
use std::fmt::format;
use std::path::{Path, PathBuf};

use regex::Regex;
use serde::Deserialize;
use serde::Serialize;

use crate::common::{McpmDataError, Mod};

#[derive(Deserialize, Serialize)]
pub struct MinecraftData {
    pub instances: Vec<MinecraftDataEntry>,
}

impl MinecraftData {
    pub fn new() -> MinecraftData {
        MinecraftData {
            instances: Vec::new()
        }
    }

    pub fn get_default_entry(&self) -> Option<MinecraftDataEntry> {
        let instances = self.instances.clone();
        let current_instance: Vec<MinecraftDataEntry> = instances.into_iter().filter(|x| { x.default == true }).collect();
        match current_instance.first() {
            None => None,
            Some(t) => Some(t.clone()),
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

    fn add_entry(&mut self, entry: MinecraftDataEntry) {
        self.instances.push(entry)
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct MinecraftInstance {
    pub path: String,
    pub loader: String,
    pub minecraft_version: String,
    pub mods: Vec<InstalledMod>,
}

impl MinecraftInstance {
    pub fn new() -> MinecraftInstance {
        MinecraftInstance {
            path: "".to_string(),
            loader: "".to_string(),
            minecraft_version: "".to_string(),
            mods: Vec::new(),
        }
    }

    pub fn create_mcpm_json(&self) -> Result<(), McpmDataError> {
        let json = serde_json::to_string(&self)?;
        let path = Path::new(&self.path).join("mcpm.json");
        fs::write(path.as_path(), json)?;
        Ok(())
    }

    pub fn parse_existing(path: String) -> Result<MinecraftInstance, McpmDataError> {
        let json = fs::read_to_string(path + "/mcpm.json")?;
        let data: MinecraftInstance = serde_json::from_str(&json)?;

        Ok(data)
    }

    pub fn add_mod(&mut self, minecraft_mod: InstalledMod) {
        self.mods.push(minecraft_mod)
    }

    pub fn save(&self) -> Result<(), McpmDataError> {
        let json = serde_json::to_string(&self)?;
        fs::write(format!("{}{}", &self.path, "/mcpm.json"), json)?;
        Ok(())
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct MinecraftDataEntry {
    pub path: String,
    pub default: bool,
}

impl MinecraftDataEntry {
    pub fn new() -> MinecraftDataEntry {
        MinecraftDataEntry {
            default: false,
            path: String::from("."),
        }
    }
    pub fn get_minecraft_instance(&self, path: &Path) -> Option<MinecraftInstance> {

        for i in MinecraftData::parse_existing(path).unwrap().instances {
            println!("{}", i.path);
            println!("{}", path.to_str().unwrap());
            if i.path == self.path {
                return Some(MinecraftInstance::parse_existing(i.path).unwrap());
            }
        }
        return None;
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct InstalledMod {
    pub name: String,
    pub slug: String,
    pub version: String,
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
    new_minecraft_instance.path = minecraft_path;

    let mut entry = MinecraftDataEntry {
        default: false,
        path: new_minecraft_instance.path.clone(),
    };

    if data.instances.is_empty() {
        entry.default = true;
    }

    new_minecraft_instance.loader = loader.trim().to_string();
    new_minecraft_instance.minecraft_version = version.trim().to_string();
    new_minecraft_instance.create_mcpm_json()?;


    data.add_entry(entry);
    data.save(data_path)?;

    Ok(())
}

