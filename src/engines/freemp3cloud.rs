use std::collections::HashMap;
use crate::types::{Engine, Music};
use crate::utils::extract_from_el;
use crate::utils::cached_reqwest;
use scraper::{Html, Selector, ElementRef};
use indicatif::ProgressBar;
use log::debug;

pub struct FreeMP3Cloud;
pub static CONFIG:Engine = Engine {
    name: "FreeMP3Cloud",
    base_url:"https://freemp3cloud.com/",
    search_url: "https://freemp3cloud.com/",
};

impl FreeMP3Cloud {
    pub async fn search(&self, _query:String) -> Result<Vec<Music>, Box<dyn std::error::Error>> {
        let _query:&str = &_query[..];
        let form_params: HashMap<&str, &str> = [("searchSong", _query)]
            .iter()
            .cloned()
            .collect();
        let bar = ProgressBar::new(100);
        let full_url: String = CONFIG.search_url.to_owned();
        let res = cached_reqwest::submit_by_gecko(&full_url, ".el-input", &form_params).await;
        let document = Html::parse_document(res.as_str());
        let selector = Selector::parse(".play-item").unwrap();
        let mut vec: Vec<Music> = Vec::new();
        let elems = document.select(&selector);
        let size = elems.count();
        for (ind, element) in document.select(&selector).enumerate() {
            let single_music = self.parse_single_music(ind, element).await;
            match single_music {
                Some(music) => vec.push(music),
                _ => (),
            }
            // increment progress bar
            let inc: u64 = (100/size) as u64;
            bar.inc(inc);
        }
        bar.finish();
        return Ok(vec)
    }

    pub async fn parse_single_music(&self, ind:usize, element:ElementRef<'_>) -> Option<Music>{
        let title = extract_from_el(&element, ".s-title", "text");
        let artiste = extract_from_el(&element, ".s-artist", "text");
        let duration = extract_from_el(&element, ".s-time", "text");
        let download_link = extract_from_el(&element, ".play-ctrl", "data-src");
        debug!("Retrieving song with title -> {}", title);
        
        Some(Music{
            index: ind+1,
            artiste: Some(artiste), 
            title, 
            download_link,
            picture_link: None,
            collection: None,
            size: None,
            duration: Some(duration),
            source: String::from(CONFIG.name).to_lowercase(),
        })
    }
}
