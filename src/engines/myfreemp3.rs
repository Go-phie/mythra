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

pub struct MyFreeMP3;
pub static CONFIG: Engine = Engine {
    name: "myfreemp3",
    base_url: "http://mp3clan.top/",
    search_url: "http://mp3clan.top/mp3/",
};

#[async_trait]
impl EngineTraits for MyFreeMP3 {
    async fn search(&self, _query: String) -> MythraResult<Vec<Music>> {
        let mut _append = str::replace(&_query[..], " ", "_");
        _append = format!("{}{}.html", CONFIG.search_url,_append);
        let mut _query= String::from(&_append[..]);

        let res = cached_reqwest::get(&_query).await;
        let selector = Selector::parse("#mp3list").unwrap();
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

impl MyFreeMP3 {
    pub async fn parse_single_music(&self, ind: usize, element: String) -> Option<Music> {
        let title_full = get_element_attribute(&element, ".unselectable", "text");
        let bitrate = get_element_attribute(&element, ".mp3list-bitrate", "text");
        let title_rg = Regex::new(r"(?P<artiste>.*) - (?P<title>.*)")
            .unwrap();
        let bitrate_rg = Regex::new(r"Check (?P<duration>.*) min")
            .unwrap();
        let title = String::from(title_rg.captures(&title_full[..])
            .unwrap()
            .name("title")
            .unwrap()
            .as_str()
            );
        let artiste = String::from(title_rg.captures(&title_full[..])
            .unwrap()
            .name("artiste")
            .unwrap()
            .as_str()
            );
        let duration = String::from(bitrate_rg.captures(&bitrate[..])
            .unwrap()
            .name("duration")
            .unwrap()
            .as_str()
            );
        let selector = Selector::parse(".mp3list-download").unwrap();
        let size = Html::parse_fragment(element.as_str())
                .select(&selector).next().unwrap().inner_html();
        let download_link = get_element_attribute(&size, "a", "href");
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
