pub mod mp3red;
use crate::utils::render_select_music;

pub async fn search_all(engine:&str, query:&str) {
    let query = String::from(query);
    match engine {
        "mp3red" => {
            let title: &str = &(format!("Searching {} for {}",
                                      engine, query.as_str()))[..];
            let e = mp3red::MP3Red{};
            let res = e.search(query);
            let results = res.await.ok().unwrap();
            render_select_music(results, title);
        },
        _ => println!("Engine is unsupported"),

    }
}
