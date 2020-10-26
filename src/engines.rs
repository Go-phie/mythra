pub mod mp3red;
use crate::types::EngineTraits;
use crate::utils::render_select_music;
use log::info;

pub fn search_all(engine:&str, query:&str) {
    let query = String::from(query);
    match engine {
        "mp3red" => {
            let title: &str = &(format!("Searching {} for {}",
                                      engine, query.as_str()))[..];
            let e = mp3red::MP3Red;
            let res = EngineTraits::search(&e, query);
            let results = res.unwrap();
            render_select_music(results, title);
            // use serde_json in api search function
//            info!("{}" ,serde_json::to_string(&results).unwrap());
        },
        _ => info!("Engine is unsupported"),

    }
}
