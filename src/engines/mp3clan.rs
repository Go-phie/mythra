use crate::types::{Engine, Music, MythraResult};
use crate::utils::cached_reqwest;
use crate::utils::extract_from_el;

use indicatif::ProgressBar;
use log::debug;
use log::info;
use scraper::{ElementRef, Html, Selector};

use std::collections::HashMap;

pub struct MP3Clan;
pub static CONFIG: Engine = Engine {
    name: "mp3clan",
    base_url: "http://mp3clan.top/",
    search_url: "http://mp3clan.top/",
};

impl MP3Clan {
    pub async fn search(&self, _query: String) -> MythraResult<Vec<Music>> {
        let mut rest_query= String::from(&_query[..]);
        rest_query = rest_query.replace(" ", "_") + ".html";
        let mut new_query = "mp3/".to_owned();
        new_query.push_str(&rest_query);
        let _query= &new_query[..];
        let form_params: HashMap<&str, &str> = [("search", _query)].iter().cloned().collect();
        let bar = ProgressBar::new(100);
        let full_url: String = CONFIG.search_url.to_owned();
        let res = cached_reqwest::_js_post(&full_url, _query, &form_params).await;
        let document = Html::parse_document(res.as_str());
        //info!("document: {:?}", document);
        info!("response -> {}", _query);
        let selector = Selector::parse(".unplaying").unwrap();
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
            let inc: u64 = (100 / size) as u64;
            bar.inc(inc);
        }
        bar.finish();

        Ok(vec)
    }

    pub async fn parse_single_music(&self, ind: usize, element: ElementRef<'_>) -> Option<Music> {
        let title = extract_from_el(&element, ".unselectable", "text");
        let duration = extract_from_el(&element, ".mp3list-bitrate", "text");
        let selector = Selector::parse(".mp3list-download").unwrap();
        let tag = element.select(&selector).next().unwrap();
        let download_link = extract_from_el(&tag, "a", "href");
        debug!("Retrieving song with title -> {}", title);

        Some(Music {
            index: ind + 1,
            artiste: None,
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
