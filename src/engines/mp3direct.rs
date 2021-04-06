use async_trait::async_trait;
use crate::types::{Engine, EngineTraits, Music, MythraResult};
use crate::utils::{
    cached_reqwest,
    get_element_attribute
};

use indicatif::ProgressBar;
use log::debug;
use scraper::{Html, Selector};
use regex::Regex;

pub struct MP3Direct;
pub static CONFIG: Engine = Engine {
    name: "mp3direct",
    base_url: "https://mp3direct.live",
    search_url: "https://mp3direct.live/#!s=",
};

#[async_trait]
impl EngineTraits for MP3Direct {
    async fn search(&self, _query: String) -> MythraResult<Vec<Music>> {
        let mut _append = str::replace(&_query[..], " ", "+");
        _append = format!("{}{}", CONFIG.search_url,_append);
        let mut _query= String::from(&_append[..]);

        let res = cached_reqwest::get(&_query, true).await;
        debug!("{:?}", res);
        let selector = Selector::parse(".ui-li").unwrap();
        let size = Html::parse_document(res.as_str())
                .select(&selector).count();
        let bar = ProgressBar::new(100);
        let other_elems: Vec<String> = Html::parse_document(res.as_str())
            .select(&selector)
            .by_ref().map(|x| x.html()).collect();
        let mut vec: Vec<Music> = Vec::new();
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

impl MP3Direct {
    pub async fn parse_single_music(&self, ind: usize, element: String) -> Option<Music> {
        let title = get_element_attribute(&element, ".ui-li-heading", "text");
        let artiste = get_element_attribute(&element, ".ui-li-desc", "text");
        let picture_link = get_element_attribute(&element, "img", "src");
        let size_full = get_element_attribute(&element, "p.source", "text");
        let size_rg = Regex::new(r"(?P<size>.*) MB")
            .unwrap();
        let size = String::from(size_rg.captures(&size_full[..])
            .unwrap()
            .name("size")
            .unwrap()
            .as_str()
            );
        let temp_dl = format!("{}{}", CONFIG.base_url, get_element_attribute(&element, "a.ui-link-inherit", "href"));
        let res = cached_reqwest::get(
            &String::from(temp_dl),
            true
            ).await;
        let download_link = get_element_attribute(&res, "#download-btn", "href");
        debug!("Retrieving song with title -> {}", title);

        Some(Music {
            index: ind + 1,
            artiste: Some(artiste),
            title,
            download_link,
            picture_link: Some(picture_link),
            collection: None,
            size: Some(size),
            duration: None,
            source: String::from(CONFIG.name).to_lowercase(),
        })
    }
}
