pub mod mp3red;
use crate::types::EngineTraits;
use crate::utils::render_select_music;
use log::info;

pub fn search_all(engine:&str, query:&str) {
    let query = String::from(query);
    match engine {
        "mp3red" => {
            let title: &str = &(format!("Searching {} for {}",
                                      engine, &query[..]))[..];
            let e = mp3red::MP3Red;
            let mut results = EngineTraits::search(&e, query)
                            .unwrap();
            render_select_music(&mut results, title);
//            info!("{}" ,serde_json::to_string(&results).unwrap());
        },
        _ => info!("Engine is unsupported"),

    }
}
