use std::fs::File;
use std::io;
use std::io::Read;

use serde::Deserialize;
use serde::Serialize;

use crate::common::{McpmDataError, Mod, ModVersion};
use crate::minecraft::{InstalledMod, MinecraftInstance};

const API_URL: &str = "https://api.modrinth.com/v2/";
const USER_AGENT: &str = "User-Agent: Handschrift/mcpm/1.0.0";

#[derive(Deserialize, Serialize)]
pub struct SearchResult {
    hits: Vec<SearchResultItem>,
}

#[derive(Deserialize, Serialize)]
pub struct SearchResultItem {
    slug: String,
    author: String,
    title: String,
    description: String,
    latest_version: String,
}

pub fn search(name: String, limit: u16) -> Result<(), McpmDataError> {
    let client = reqwest::blocking::Client::new();

    let request_url = String::from(API_URL)
        + "search?query=" + &name
        + "&limit=" + &limit.to_string()
        + "&facets=[[\"project_type:mod\"]]";

    let mut res = client.get(request_url).header(reqwest::header::USER_AGENT, USER_AGENT).send()?;

    let mut body = String::new();
    res.read_to_string(&mut body)?;

    let search_result: SearchResult = serde_json::from_str(&body)?;

    for hit in search_result.hits {
        println!("{}\t{}\tVersion: {}\t (slug: {})", hit.title, hit.description, hit.latest_version, hit.slug);
    }

    Ok(())
}

pub fn download(mod_slug: String, mut minecraft_instance: MinecraftInstance) -> Result<(), McpmDataError> {
    let minecraft_mod = get_mod(mod_slug)?;

    let versions = get_mod_versions(minecraft_mod.versions)?;

    let potential_versions: Vec<ModVersion> = versions.into_iter().filter(|version| {
        version.loaders.contains(&minecraft_instance.loader) && version.game_versions.contains(&minecraft_instance.minecraft_version)
    }).collect();

    //get the last element to get the newest version of the mod
    match potential_versions.last() {
        Some(version) => {
            for v in &version.files {
                if v.primary {
                    let client = reqwest::blocking::Client::new();
                    let mut res = client.get(&v.url).send()?;
                    let mut file = File::create(String::from("mods/") + &v.filename)?;
                    let installed_minecraft_mod = InstalledMod {
                        version: v.filename.clone(),
                        slug: minecraft_mod.slug.clone(),
                        name: minecraft_mod.title.clone(),
                    };
                    io::copy(&mut res, &mut file)?;
                    minecraft_instance.add_mod(installed_minecraft_mod);
                    minecraft_instance.save().expect("Couldn't save!");
                    println!("Installed: {}", v.filename);
                }
            }
        }
        None => println!("No versions matched the specified constraints")
    };

    Ok(())
}

pub fn get_mod_versions(version_ids: Vec<String>) -> Result<Vec<ModVersion>, McpmDataError> {
    let client = reqwest::blocking::Client::new();

    let request_url = String::from(API_URL)
        + "versions?ids=" + "[\"" + &version_ids.join("\",\"") + "\"]";

    let mut res = client.get(request_url).header(reqwest::header::USER_AGENT, USER_AGENT).send()?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    let versions: Vec<ModVersion> = serde_json::from_str(&body)?;
    return Ok(versions);
}

pub fn get_mod(mod_slug: String) -> Result<Mod, McpmDataError> {
    let client = reqwest::blocking::Client::new();

    let request_url = String::from(API_URL)
        + "project/" + &mod_slug;

    let mut res = client.get(request_url).header(reqwest::header::USER_AGENT, USER_AGENT).send()?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    let minecraft_mod: Mod = serde_json::from_str(&body)?;


    Ok(minecraft_mod)
}