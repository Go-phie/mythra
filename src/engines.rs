pub mod mp3red;
use crate::types::EngineTraits;

#[allow(dead_code)]
pub fn search_all() {
    let e = mp3red::MP3Red;
    let engine = EngineTraits::search(&e, "broken mirrors".to_string());
    println!("{:?}", engine)
}
