use serde::{Serialize, Deserialize};
use scraper::ElementRef;
use std::borrow::Borrow;

#[derive(Serialize, Deserialize, Debug)]
pub struct Music {
    // Option<> elements are struct elements
    // that are allowed to be None
    pub index: usize,
    pub artiste: Option<String>,
    pub title: String,
    pub collection: Option<String>,
    pub download_link: String,
    pub picture_link: Option<String>,
    pub size: Option<String>,
    pub duration: Option<String>,
    pub source: String,
}

// Enables borrow due to cursive cli
impl<'a> Borrow<Music> for &mut &Music {
    fn borrow(&self) -> &Music {
        self
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Engine {
    pub name: &'static str,
    pub base_url: &'static str,
    pub search_url: &'static str,
}


pub trait EngineTraits {
    fn search(&self, query:String) -> Result<Vec<Music>, Box<dyn std::error::Error>>;
    fn parse_single_music(&self, ind:usize, el:ElementRef) -> Result<Music, Box<dyn std::error::Error>>;
}

// Music request parser for API
#[derive(Deserialize)]
pub struct MusicRequest {
    pub query: String,
    pub engine: String,
}
