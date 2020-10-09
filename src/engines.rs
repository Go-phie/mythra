pub mod mp3red;
use crate::types::EngineTraits;

pub fn search_all() {
    let e = mp3red::MP3Red;
    let engine = EngineTraits::search(&e, "broken mirrors".to_string());
    match engine {
        Ok(music_array) => {
            println!("{}", serde_json::to_string(&music_array).unwrap());
        },
        _ => (),
    }
}
