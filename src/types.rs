use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Music {
    pub index: u32,
    pub artiste: Option<String>,
    pub title: String,
    pub collection: Option<String>,
    pub download_link: String,
    pub picture_link: Option<String>,
    pub source: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Engine {
    pub name: &'static str,
    pub base_url: &'static str,
    pub search_url: &'static str,
}


pub trait EngineTraits {
    fn search(&self, query:String) -> Vec<Music>;
    fn parse_single_music(&self) -> Music;
}
