use crate::types::{Engine, EngineTraits, Music};

pub struct RedThreeMP3;
static CONFIG:Engine = Engine {
    name: "RedThreeMP3",
    base_url:"https://red3mp3.me",
    search_url: "https://red3mp3.me/Search",
};

impl EngineTraits for RedThreeMP3 {
    fn search(&self, _query:String) -> Vec<Music> {
        return vec![Music{
            index: 1,
            artiste: Some(String::from("None")), 
            title: String::from("whaddup cisgendered niggAs"),
            download_link: String::from("whaddup cisgendered niggAs"),
            picture_link: None,
            collection: None,
            source: String::from(CONFIG.name).to_lowercase(),
//            source: String::from(self.engine.name)
        }]
    }
}
