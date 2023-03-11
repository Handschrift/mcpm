use std::cmp::min;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Seek;
use std::io::Write;
use std::process::exit;

use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use serde::Deserialize;
use serde::Serialize;

use crate::common::{McpmDataError, Mod, ModVersion};
use crate::minecraft::{InstalledMod, MinecraftInstance};

const API_URL: &str = "https://api.modrinth.com/v2/";
const USER_AGENT: &str = "User-Agent: Handschrift/mcpm/0.2.0";

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

pub async fn search(name: String, limit: u16) -> Result<(), McpmDataError> {
    let client = Client::new();

    let request_url = String::from(API_URL)
        + "search?query=" + &name
        + "&limit=" + &limit.to_string()
        + "&facets=[[\"project_type:mod\"]]";

    let res = client.get(request_url).header(reqwest::header::USER_AGENT, USER_AGENT).send().await?;

    let body = res.text().await?;

    let search_result: SearchResult = serde_json::from_str(&body)?;

    for hit in search_result.hits {
        println!("{}\t{}\tVersion: {}\t (slug: {})", hit.title, hit.description, hit.latest_version, hit.slug);
    }

    Ok(())
}

pub async fn download_file(client: &Client, url: &str, path: &str) -> Result<(), String> {
    let res = client
        .get(url)
        .send()
        .await
        .or(Err(format!("Failed to GET from '{}'", &url)))?;
    let total_size = res
        .content_length()
        .ok_or(format!("Failed to get content length from '{}'", &url))?;

    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.white/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
        .unwrap()
        .progress_chars("#  "));
    pb.set_message(format!("Downloading {}", url));

    let mut file;
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    if std::path::Path::new(path).exists() {
        file = OpenOptions::new()
            .read(true)
            .append(true)
            .open(path)
            .unwrap();

        let file_size = std::fs::metadata(path).unwrap().len();
        file.seek(std::io::SeekFrom::Start(file_size)).unwrap();
        downloaded = file_size;
    } else {
        file = File::create(path).or(Err(format!("Failed to create file '{}'", path)))?;
    }

    while let Some(item) = stream.next().await {
        let chunk = item.or(Err("Error while downloading file"))?;
        file.write(&chunk)
            .or(Err("Error while writing to file"))?;
        let new = min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;
        pb.set_position(new);
    }
    return Ok(());
}


pub async fn download(mod_slug: String, mut minecraft_instance: MinecraftInstance) -> Result<(), McpmDataError> {
    let client = Client::new();
    let minecraft_mod = get_mod(&client, mod_slug).await?;
    let versions = get_mod_versions(&client, minecraft_mod.versions).await?;
    let potential_versions: Vec<ModVersion> = versions.into_iter().filter(|version| {
        version.loaders.contains(&minecraft_instance.loader) && version.game_versions.contains(&minecraft_instance.minecraft_version)
    }).collect();
    if minecraft_instance.mods.iter().any(|x| { x.slug == minecraft_mod.slug }) {
        println!("The mod {} is already installed.", &minecraft_mod.title);
        return Ok(());
    }
    //get the last element to get the newest version of the mod
    match potential_versions.last() {
        Some(version) => {
            for v in &version.files {
                if v.primary {
                    download_file(&client, &v.url, format!("mods/{}", &v.filename).as_str()).await.unwrap();

                    println!("Updating lockfile...");
                    let installed_minecraft_mod = InstalledMod {
                        version: v.filename.clone(),
                        slug: minecraft_mod.slug.clone(),
                        name: minecraft_mod.title.clone(),
                    };
                    minecraft_instance.add_mod(installed_minecraft_mod);
                    minecraft_instance.save().expect("Couldn't save!");
                    println!("{} has been successfully installed!", v.filename);
                }
            }
        }
        None => {
            println!("No versions matched the specified constraints");
        }
    };

    Ok(())
}

pub async fn get_mod_versions(client: &Client, version_ids: Vec<String>) -> Result<Vec<ModVersion>, McpmDataError> {
    let request_url = String::from(API_URL)
        + "versions?ids=" + "[\"" + &version_ids.join("\",\"") + "\"]";

    let res = client.get(request_url).header(reqwest::header::USER_AGENT, USER_AGENT).send().await?;
    let body = res.text().await?;

    let versions: Vec<ModVersion> = serde_json::from_str(&body)?;
    return Ok(versions);
}

pub async fn get_mod(client: &Client, mod_slug: String) -> Result<Mod, McpmDataError> {
    let request_url = String::from(API_URL)
        + "project/" + &mod_slug;

    let res = client.get(request_url).header(reqwest::header::USER_AGENT, USER_AGENT).send().await?;
    let body = res.text().await?;

    let minecraft_mod: Mod = serde_json::from_str(&body)?;


    Ok(minecraft_mod)
}