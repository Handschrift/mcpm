use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize)]
pub struct MinecraftData {
    pub path: String,
}

impl MinecraftData {
    fn new() -> MinecraftData {
        MinecraftData {
            path: "".to_string()
        }
    }
}


pub fn init(minecraft_path: String) {}