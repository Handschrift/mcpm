use std::io;

use clap::builder::Str;
use serde::{Deserialize, Serialize};
use serde_json::Error;

use crate::common::McpmDataError::{FileSystemError, JsonError, NetworkError};

#[derive(Deserialize, Serialize, Clone)]
pub struct Mod {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub project_type: String,
    pub versions: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ModVersion {
    pub id: String,
    pub game_versions: Vec<String>,
    pub loaders: Vec<String>,
    pub files: Vec<ModFile>,
    pub dependencies: Vec<Dependency>,
    pub date_published: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Dependency {
    pub version_id: Option<String>,
    pub project_id: Option<String>,
    pub dependency_type: String,
}


#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ModFile {
    pub url: String,
    pub filename: String,
    pub size: u32,
    pub primary: bool,
}

#[derive(Debug)]
pub enum McpmDataError {
    JsonError(Error),
    FileSystemError(io::Error),
    NetworkError(reqwest::Error),
}

impl From<Error> for McpmDataError {
    fn from(err: Error) -> Self {
        JsonError(err)
    }
}

impl From<io::Error> for McpmDataError {
    fn from(err: io::Error) -> Self {
        FileSystemError(err)
    }
}

impl From<reqwest::Error> for McpmDataError {
    fn from(err: reqwest::Error) -> Self {
        NetworkError(err)
    }
}