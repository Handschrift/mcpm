use std::io;
use serde::{Deserialize, Serialize};
use serde_json::Error;
use crate::common::McpmDataError::{FileSystemError, JsonError, NetworkError};

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
    pub date_published: String,
}

#[derive(Deserialize, Serialize)]
pub struct ModFile {
    pub url: String,
    pub filename: String,
    pub size: u32,
}

pub enum McpmDataError {
    JsonError(serde_json::Error),
    FileSystemError(io::Error),
    NetworkError(reqwest::Error),
}

impl From<serde_json::Error> for McpmDataError {
    fn from(err: Error) -> Self {
        JsonError(err)
    }
}

impl From<io::Error> for McpmDataError {
    fn from(err: std::io::Error) -> Self {
        FileSystemError(err)
    }
}

impl From<reqwest::Error> for McpmDataError {
    fn from(err: reqwest::Error) -> Self {
        NetworkError(err)
    }
}