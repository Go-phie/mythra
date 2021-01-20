use crate::types::{Engine, Music, MythraResult};
use crate::utils::cached_reqwest;
use crate::utils::extract_from_el;

use indicatif::ProgressBar;
use log::debug;
use scraper::{ElementRef, Html, Selector};


pub struct MyFreeMP3;
pub static CONFIG: Engine = Engine {
    name: "myfreemp3",
    base_url: "http://mp3clan.top/",
    search_url: "https://my-free-mp3.vip/api/search.php",
};

impl MyFreeMP3 {
    pub async fn search(&self, _query: String) -> MythraResult<Vec<Music>> {
        let mut _query= String::from(&_query[..]);
        let bar = ProgressBar::new(100);
        let full_url: String = CONFIG.search_url.to_owned();
        let res = cached_reqwest::post(&full_url,&_query).await;
        debug!("Retrieving song with response -> {:?}", res);
        let document = Html::parse_document("test");
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
