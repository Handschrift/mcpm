use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Mod {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub project_type: String,
    pub versions: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct ModVersion {
    pub id: String,
    pub game_versions: Vec<String>,
    pub loaders: Vec<String>,
    pub files: Vec<ModFile>,
}

#[derive(Deserialize, Serialize)]
pub struct ModFile {
    pub url: String,
    pub filename: String,
    pub size: u32,
}