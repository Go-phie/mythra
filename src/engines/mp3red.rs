use crate::types::{Engine, EngineTraits, Music};
//use std::collections::HashMap;

pub struct MP3Red;
static CONFIG:Engine = Engine {
    name: "MP3Red",
    base_url:"https://mp3red.best/",
    search_url: "https://mp3red.best/mp3/",
};

impl EngineTraits for MP3Red {
    fn search(&self, _query:String) -> Vec<Music> {
        let _query:&str = &_query.replace(" ", "-")[..];
        let mut full_url: String = CONFIG.search_url.to_owned();
        full_url.push_str(_query);
        let client = reqwest::blocking::Client::new();
        let res = client.get(&full_url).send();
        println!("{:?}", res.json());
        let single_music = self.parse_single_music();
        return vec![single_music]
    }

    fn parse_single_music(&self) -> Music {
        Music{
            index: 1,
            artiste: None, 
            title: String::from("whaddup cisgendered niggAs"),
            download_link: String::from("whaddup cisgendered niggAs"),
            picture_link: None,
            collection: None,
            source: String::from(CONFIG.name).to_lowercase(),
        }
    }
}
