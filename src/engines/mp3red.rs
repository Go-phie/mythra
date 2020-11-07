use crate::types::{Engine, EngineTraits, Music};
use crate::utils::extract_from_el;
use crate::utils::cached_reqwest;
use scraper::{Html, Selector, ElementRef};
use indicatif::ProgressBar;

pub struct MP3Red;
pub static CONFIG:Engine = Engine {
    name: "MP3Red",
    base_url:"https://mp3red.best/",
    search_url: "https://mp3red.best/mp3/",
};

impl EngineTraits for MP3Red {
    fn search(&self, _query:String) -> Result<Vec<Music>, Box<dyn std::error::Error>> {
        let _query:&str = &_query.replace(" ", "-")[..];
        let bar = ProgressBar::new(100);
        let mut full_url: String = CONFIG.search_url.to_owned();
        full_url.push_str(_query);
        let res = cached_reqwest::get(&full_url);
        let document = Html::parse_document(res.as_str());
        let selector = Selector::parse("div.box-post").unwrap();
        let mut vec: Vec<Music> = Vec::new();
        let elems = document.select(&selector);
        let size = elems.count();
        for (ind, element) in document.select(&selector).enumerate() {
            let single_music = self.parse_single_music(ind, element);
            match single_music {
                Ok(music) => vec.push(music),
                _ => (),
            }
            // increment progress bar
            let inc: u64 = (100/size) as u64;
            bar.inc(inc);
        }
        bar.finish();

        return Ok(vec)
    }

    fn parse_single_music(&self, ind:usize, element:ElementRef) -> Result<Music, Box<dyn std::error::Error>> {
        let picture_link = extract_from_el(&element, "img", "src");
        let title = extract_from_el(&element, "img", "alt");
        let duration = extract_from_el(&element, ".duration", "text");
        let size = extract_from_el(&element, ".file-size", "text");
        let initial_download_link = extract_from_el(&element,".pull-left","href");
        let res = cached_reqwest::get(&initial_download_link);
        let document = Html::parse_document(res.as_str());
        let dl_selector = Selector::parse(".dl-list").unwrap();
        let dl_element = document.select(&dl_selector).next().unwrap();
        let download_link = extract_from_el(&dl_element, "[class='btn']", "href");
        
        Ok(Music{
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
