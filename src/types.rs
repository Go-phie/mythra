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

pub struct Engine {
    pub name: String,
    pub base_url: String,
    pub search_url: String,
}


pub trait EngineTraits {
    fn new(&mut self);
    fn search(&self, query:String) -> Vec<Music>;
    //    fn parse_single_music(&self, index: u32, soup: String) -> Music;
}
