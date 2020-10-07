use crate::types::{Engine, EngineTraits, Music};

pub struct RedThreeMP3{
    engine: Engine,
}

impl EngineTraits for RedThreeMP3 {
    fn new(&mut self) {
        self.engine = Engine{
            name: String::from("RedThreeMP3"),
            base_url:String::from("https://red3mp3.me"),
            search_url: String::from("https://red3mp3.me/Search"),
        };
    }

    fn search(&self, _query:String) -> Vec<Music> {
        return vec![Music{
            index: 1,
            artiste: Some(String::from("None")), 
            title: String::from("whaddup cisgendered niggAs"),
            download_link: String::from("whaddup cisgendered niggAs"),
            picture_link: None,
            collection: None,
            source: String::from("redthreemp3"),
//            source: String::from(self.engine.name)
        }]
    }
}
