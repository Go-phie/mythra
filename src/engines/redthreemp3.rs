use crate::types::{Engine, EngineTraits, Music};

pub struct RedThreeMP3;
static CONFIG:Engine = Engine {
    name: "RedThreeMP3",
    base_url:"https://red3mp3.me",
    search_url: "https://red3mp3.me/Search",
};

impl EngineTraits for RedThreeMP3 {
    fn search(&self, _query:String) -> Vec<Music> {
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
