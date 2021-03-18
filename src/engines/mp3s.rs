use mockall::*;
use async_trait::async_trait;
use crate::types::{Engine, Music, MythraResult, EngineTraits};
use crate::utils::cached_reqwest;
use crate::utils::get_element_attribute;

use indicatif::ProgressBar;
use log::debug;
use scraper::{Html, Selector};

use std::collections::HashMap;

#[derive(Clone)]
pub struct MP3S;
pub static CONFIG: Engine = Engine {
    name: "MP3S",
    base_url: "https://freemp3cloud.com/es/",
    search_url: "https://freemp3cloud.com/es/",
};

#[async_trait]
impl EngineTraits for MP3S {
    async fn search(&self, _query: String) -> MythraResult<Vec<Music>> {
        let _query: &str = &_query[..];
        let form_params: HashMap<&str, &str> = [("searchSong", _query)].iter().cloned().collect();
        let bar = ProgressBar::new(100);
        let full_url: String = CONFIG.search_url.to_owned();
        let res = cached_reqwest::js_post(&full_url, ".el-input", &form_params).await;
        let selector = Selector::parse(".play-item").unwrap();
        let mut vec: Vec<Music> = Vec::new();
        let size = Html::parse_document(res.as_str())
                .select(&selector).count();
        let other_elems: Vec<String> = Html::parse_document(res.as_str())
            .select(&selector)
            .by_ref().map(|x| x.html()).collect();

        for el in 0..size {
            let element = &other_elems[el];
            let single_music = self.parse_single_music(el, element.to_string()).await;
            match single_music {
                Some(music) => vec.push(music),
                _ => (),
            }
//             increment progress bar
            let inc: u64 = (100 / size) as u64;
            bar.inc(inc);
        };
        bar.finish();

        Ok(vec)
    }


}

#[automock]
impl MP3S {
    pub async fn parse_single_music(&self, ind: usize, element: String) -> Option<Music> {
        let title = get_element_attribute(&element, ".s-title", "text");
<<<<<<< HEAD
        let artiste = get_element_attribute(&element, ".s-artist", "text");
        let duration = get_element_attribute(&element, ".s-time", "text");
        let download_link = get_element_attribute(&element, "[target='_blank'", "href");
=======
        let artiste = get_element_attribute(&element, ".s-artiste", "text");
        let duration = get_element_attribute(&element, ".s-time", "text");
        let download_link = get_element_attribute(&element, ".s-time", "data-src");
>>>>>>> 53e0e3c1ae7b9f2e9e6f1aa4a039ead522a28870
        debug!("Retrieving song with title -> {}", title);

        Some(Music {
            index: ind + 1,
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
