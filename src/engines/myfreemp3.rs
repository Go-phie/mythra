use crate::types::{Engine, Music, MythraResult};
use crate::utils::cached_reqwest;
use crate::utils::extract_from_el;

use indicatif::ProgressBar;
use log::{debug, info};
use scraper::{ElementRef, Html, Selector};
use serde_json::{Value};
use serde::{Deserialize, Serialize};
use serde_json::Result;


pub struct MyFreeMP3;
pub static CONFIG: Engine = Engine {
    name: "myfreemp3",
    base_url: "http://mp3clan.top/",
    search_url: "https://my-free-mp3.vip/api/search.php",
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    response: Vec<Test>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Arr {
    First(String),
    Vector(Test)
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Test {
   name: String
}
#[derive(Serialize, Deserialize, Debug)]
struct Elem {
    artist: String,
    id: u64,
    owner_id: u64,
    title: String,
    access_key: String,
    is_licenced: bool,
    url: String,
    date: u64,
    album: Album,
    short_video_allowed: bool,
    stories_allowed: bool,
    stories_cover_allowed: bool
}
#[derive(Serialize, Deserialize, Debug)]
struct Album {
    id: usize,
    title: String,
    owner_id: i64,
    access_key: String,
    thumb: Thumb
}
#[derive(Serialize, Deserialize, Debug)]
struct Thumb {
    width: i64,
    height: i64,
    photo_34: String,
    photo_68: String,
    photo_135: String,
    photo_270: String,
    photo_300: String,
    photo_600: String,
    photo_1200: String,
}

impl MyFreeMP3 {
    pub async fn search(&self, _query: String) -> MythraResult<Vec<Music>> {
        let mut _query= String::from(&_query[..]);
        let bar = ProgressBar::new(100);
        let full_url: String = CONFIG.search_url.to_owned();
        let form_data = [
                        ("q", _query.as_str()),
                        ("sort", "2"),
                        ("page", "0"),
                    ];
        let res = cached_reqwest::post(&full_url,&form_data).await.ok().unwrap();
        //info!("Retrieving song with response -> {:?}", res);
        let v: Data = self.typed_example(&res).ok().unwrap();
        println!("Retrieving song with data -> {:?}", v);
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
    pub fn typed_example(&self, data: &String) -> Result<Data> {
        let data = r#"
            {
                "response": [
                    "apple",
                    {
                        "name": "wisdom"
                    }
                ]
            }"#;
        let d: Data = serde_json::from_str(data)?;

        // Do things just like with any other Rust data structure.
        println!("Data {:?}", d);

        Ok(d)
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
