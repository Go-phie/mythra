use std::collections::HashMap;
use crate::types::{Engine, Music};
use crate::utils::extract_from_el;
use crate::utils::cached_reqwest;
use scraper::{Html, Selector, ElementRef};
use indicatif::ProgressBar;

pub struct MP3Tomato;
pub static CONFIG:Engine = Engine {
    name: "MP3Tomato",
    base_url:"https://mp3tomato.com/",
    search_url: "https://mp3tomato.com/",
};

impl MP3Tomato {
    pub async fn search(&self, _query:String) -> Result<Vec<Music>, Box<dyn std::error::Error>> {
        let _query:&str = &_query[..];
        let mut form_params = HashMap::new();
        form_params.insert("s", _query);
        let bar = ProgressBar::new(100);
        let full_url: String = CONFIG.search_url.to_owned();
        let res = cached_reqwest::post(&full_url, &form_params).await;
        let document = Html::parse_document(res.as_str());
        let selector = Selector::parse("div.box-post").unwrap();
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
        let picture_link = extract_from_el(&element, "img", "src");
        let title = extract_from_el(&element, "img", "alt");
        let duration = extract_from_el(&element, ".duration", "text");
        let size = extract_from_el(&element, ".file-size", "text");
        let initial_download_link = extract_from_el(&element,".pull-left","href");
        let res = cached_reqwest::get(&initial_download_link).await;
        let document = Html::parse_document(res.as_str());
        let dl_selector = Selector::parse(".dl-list").unwrap();
        let dl_element = document.select(&dl_selector).next().unwrap();
        let download_link = extract_from_el(&dl_element, "[class='btn']", "href");
        
        Some(Music{
            index: ind+1,
            artiste: None, 
            title, 
            download_link,
            picture_link: Some(picture_link),
            collection: None,
            size: Some(size),
            duration: Some(duration),
            source: String::from(CONFIG.name).to_lowercase(),
        })
    }
}
