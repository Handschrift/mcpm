use std::error::Error;
use std::io::Read;

use serde::Deserialize;
use serde::Serialize;

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



pub fn search(name: String, limit: u16) -> Result<(), Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();

    let request_url = String::from(API_URL)
        + "search?query=" + &name
        + "&limit=" + &limit.to_string();

    let mut res = client.get(request_url).header(reqwest::header::USER_AGENT, USER_AGENT).send()?;

    let mut body = String::new();
    res.read_to_string(&mut body)?;

    let search_result: SearchResult = serde_json::from_str(&body)?;

    for hit in search_result.hits {
        println!("{}\t{}\tVersion: {}", hit.title, hit.description, hit.latest_version);
    }

    Ok(())
}

pub fn download() {}