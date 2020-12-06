pub mod mp3red;
pub mod freemp3cloud;
use crate::utils::render_select_music;
use crate::types::Music;
use log::error;

pub async fn search_all(engine:&str, query:&str) -> Result<Vec<Music>, Box<dyn std::error::Error>> {
    let query = String::from(query);
    match engine {
        "mp3red" => {
            let e = mp3red::MP3Red{};
            e.search(query).await
        },
        "freemp3cloud" => {
            let e = freemp3cloud::FreeMP3Cloud{};
            e.search(query).await
        },
        _ => {
            let empty: Vec<Music> = vec![];
            error!("Engine is unsupported");
            Ok(empty)
        },

    }
}

pub async fn cli(engine: &str, query:&str){
    let title: &str = &(format!("Searching {} for {}",
                                engine, query))[..];
    let results = search_all(engine, query).await.ok().unwrap();
    render_select_music(results, title);
}
