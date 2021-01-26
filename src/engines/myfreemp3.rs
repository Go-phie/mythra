use serde_json::Map;
use crate::types::{Engine, Music, MythraResult};
use crate::utils::cached_reqwest;

use indicatif::ProgressBar;
use log::debug;
use serde_json::{Value};
use serde_json::Result;
use regex::Regex;

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
        let form_data = [
                        ("q", _query.as_str()),
                        ("sort", "2"),
                        ("page", "0"),
                    ];
        let res = cached_reqwest::post(
            &full_url,
            &form_data
            ).await.unwrap();
        let v: Value = self.format_response(&res).ok().unwrap().clone();
        //println!("Retrieving song with data -> {:?}", v);
        let mut vec: Vec<Music> = Vec::new();
        let elems = v["response"].as_array().unwrap();
        let clone = elems.clone();
        let size = clone.len();
        for (ind, element) in clone.iter().enumerate() {
            let single_music = self.parse_single_music(ind, &element.as_object()).await;
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

    pub fn format_response(&self, data: &String) -> Result<Value> {
        let new_data = data.as_str();
        let new_data = &new_data.replace("\"apple\",", "");
        //println!("Data response: {:?}", new_data);
        let re = Regex::new(r"^(?P<last>[(])(?P<content>.*)(?P<first>[)][;]$)").unwrap();
        let result = re.replace(new_data.as_str(), "$content");
        //println!("Data response: {:?}", result);
        let d: Value = serde_json::from_str(result.into_owned().as_str()).unwrap();
        // Do things just like with any other Rust data structure.
        Ok(d)
    }

    pub async fn parse_single_music(&self,ind :usize, element: &Option<&Map<String, Value>>) -> Option<Music> {
        let title = element.unwrap().get("title").unwrap().as_str().unwrap().to_owned();
        let artiste = element.unwrap().get("artist").unwrap().as_str().unwrap().to_owned();
        let data = r#" { "album": {"title": "-"} } "#;
        let def_album:Value = serde_json::from_str(data).unwrap();
        let album = element.unwrap().get("album").unwrap_or(
            &def_album["album"]
            ).as_object();
        let collection = album.unwrap().get("title").unwrap().as_str().unwrap().to_owned();
        let duration_i64 = element.unwrap().get("duration").unwrap().as_i64().unwrap();
        let duration = (format!("{}:{}", duration_i64/60, duration_i64%60)).into();
        let download_link = element.unwrap().get("url").unwrap().as_str().unwrap().to_owned();
        debug!("Retrieving song with title -> {}", title);
        debug!("Artiste -> {}", artiste);
        debug!("Download url -> {}", download_link);
        debug!("Duration -> {}", duration);
        debug!("Collection -> {:?}", collection);

        Some(Music {
            index: ind + 1,
            artiste: Some(artiste),
            title,
            download_link,
            picture_link: None,
            collection: Some(collection),
            size: None,
            duration: Some(duration),
            source: String::from(CONFIG.name).to_lowercase(),
        })
    }
}
