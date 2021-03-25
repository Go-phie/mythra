use crate::download::{download_from_url, download_size};
use crate::types::{Music, MythraResult};

use cursive::align::HAlign;
use cursive::view::{Resizable, Scrollable};
use cursive::views::{Dialog, ProgressBar, SelectView};
use cursive::Cursive;

use env_logger::Builder;
use fantoccini;
use log::{debug, info};

use scraper::{Html, Selector};
use std::collections::{hash_map::DefaultHasher, HashMap};
use std::env;
use std::fs;
use std::hash::{Hash, Hasher};
use std::io::{Read, Write};
use std::path::Path;

pub static CACHE_NAME: &str = ".mythra-cache";

pub fn get_element_attribute(element: &String, selector: &str,  attribute: &str) -> String {
    let document = Html::parse_document(element.as_str());
    let  selector = Selector::parse(selector).unwrap();
    match attribute {
        "text" => {
            document.select(&selector)
            .next().unwrap().text().collect::<String>()
        }
        attr => {
            document.select(&selector)
            .next().unwrap().value().attr(attr).unwrap().to_owned()

        }

    }
}


// Removes cache directory
pub fn clear_cache() {
    match env::current_exe() {
        Ok(current_exe) => {
            let path: &Path = Path::new(current_exe.to_str().unwrap());
            let parent: &str = path.parent().unwrap().to_str().unwrap();
            let full_dir_path = format!("{}/{}", parent, crate::utils::CACHE_NAME);
            match fs::remove_dir_all(full_dir_path) {
                Ok(_) => {
                    info!("Mythra cache cleared!");
                }
                Err(_) => {
                    info!("Mythra cache does not exist");
                }
            }
        }
        Err(_) => (),
    }
}

pub fn render_select_music(songs: Vec<Music>, title: &str) {
    let mut select = SelectView::new().h_align(HAlign::Center).autojump();
    for song in songs {
        let title_copy = &song.title.clone();
        let title = title_copy.as_str();
        select.add_item(title, song);
    }
    let mut siv = cursive::default();
    select.set_on_submit(render_downloading_song);
    // Let's add a ResizedView to keep the
    // list at a reasonable size (it can scroll anyway).
    siv.add_layer(Dialog::around(select.scrollable().fixed_size((50, 10))).title(title));
    siv.run();
}

fn render_downloading_song(siv: &mut Cursive, song: &Music) {
    //    replace previous view
    let link_copy = song.download_link.clone();
    let use_link = &link_copy[..];
    let download_size_u64 = download_size(use_link).unwrap();
    let cb = siv.cb_sink().clone();
    siv.add_layer(
        Dialog::around(
            ProgressBar::new()
                .range(0, download_size_u64 as usize)
                .with_task(move |counter| {
                    download_from_url(counter, link_copy.to_owned());
                    cb.send(Box::new(completed_download)).unwrap();
                })
                .full_width(),
        )
        .button("Minimize", |siv| {
            siv.pop_layer();
        }),
    );
    siv.set_autorefresh(true);
}

// complete download callback
fn completed_download(siv: &mut Cursive) {
    siv.set_autorefresh(false);
    siv.pop_layer();
    siv.add_layer(
        Dialog::new()
            .title("Download complete")
            .button("Return", |siv| {
                siv.pop_layer();
            }),
    );
}

// configure logging
pub fn configure_log(level: &str) {
    let logname: &str = "MYTHRA_LOG_FMT";
    // activate debugging for only actix_web and mythra
    let logfmt = format!("actix_web={},mythra={}", level, level);
    env::set_var(logname, logfmt);
    Builder::new().parse_env(logname).init();
}

// Wrapper around the reqwest module
// Retrieve web pages from cache if they exist
// else retrieve from url

pub mod cached_reqwest {
    #[allow(dead_code)]
    use super::*;
    

    pub fn create_or_retrieve(
        url: String,
        exe_path: std::path::PathBuf,
    ) -> (std::fs::File, String) {
        let path: &Path = Path::new(exe_path.to_str().unwrap());
        let parent: &str = path.parent().unwrap().to_str().unwrap();
        // hash url
        let mut hasher = DefaultHasher::new();
        url.hash(&mut hasher);
        let hashed_url: &str = &(hasher.finish().to_string())[..];
        let full_dir_path = format!("{}/{}", parent, crate::utils::CACHE_NAME);
        let full_path = format!("{}/{}", full_dir_path, hashed_url);
        // create all parent directories necessary
        fs::create_dir_all(full_dir_path).ok().unwrap();
        let mut file = fs::OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open(full_path)
            .unwrap();
        // read file contents to String
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        //                Ok((file, contents))
        (file, contents)
    }

    pub async fn fantoccini_form(
        url: &String,
        form_selector: &str,
        params: &HashMap<&str, &str>,
    ) -> MythraResult<String> {
        let results = match env::current_exe() {
            Ok(exe_path) => {
                let mut val_map = String::from("");
                for (key, val) in params {
                    val_map = val_map + &(format!("{}={}", key, val))[..]
                }
                let concat_url = url.to_owned() + &val_map[..];
                let (mut file, contents) = create_or_retrieve(concat_url, exe_path);
                // if file is empty then cache does not exist
                // then retrieve directly using reqwest
                let mut caps = serde_json::map::Map::new();
                let opts = serde_json::json!({ "args": [
                    "--headless", 
                    "--disable-gpu", 
                    "--no-sandbox", 
                    "--disable-dev-shm-usage",
                    ] });
                caps.insert("goog:chromeOptions".to_string(), opts.clone());
                if (contents.as_str()).eq("") {
                    let mut c = fantoccini::Client::with_capabilities("http://localhost:4444", caps)
                        .await
                        .expect("failed to connect to WebDriver");
                    c.goto(url).await.unwrap();
                    let mut form = c
                        .form(fantoccini::Locator::Css(form_selector))
                        .await
                        .unwrap();
                    for (key, val) in params {
                        form.set_by_name(key, val).await.unwrap();
                    }
                    let mut res_client = form.submit().await.unwrap();
                    let res = res_client.source().await.unwrap();
                    file.write_all((res.as_str()).as_bytes())?;
                    c.close().await.unwrap();
                    debug!("Retrieving {} [POST] data from web (fantoccini)", url);
                    res
                } else {
                    debug!("Retrieving {} [POST] data from cache (fantoccini)", url);
                    contents
                }
            }
            Err(e) => {
                return Err(Box::new(e));
                // format!("failed to get current exe path: {}", e);
            }
        };
        Ok(results)
    }

    pub async fn getter(url: &String) -> MythraResult<String> {
        let mut results = String::new();
        match env::current_exe() {
            Ok(exe_path) => {
                let (mut file, contents) = create_or_retrieve(url.to_string(), exe_path);
                // if file is empty then cache does not exist
                // then retrieve directly using reqwest
                if (contents.as_str()).eq("") {
                    let res = reqwest::get(url).await.unwrap().text().await.unwrap();
                    file.write_all((res.as_str()).as_bytes())?;
                    debug!("Retrieving {} [GET] data from web", url);
                    results = res;
                } else {
                    debug!("Retrieving {} [GET] data from cache", url);
                    results = contents;
                }
            }
            Err(e) => {
                format!("failed to get current exe path: {}", e);
            }
        };
        Ok(results)
    }

    pub async fn get(url: &String) -> String {
        let new_url = url.clone();
        getter(&new_url).await.ok().unwrap()
    }

    pub async fn post(url: &String, params: &[(&str, &str)]) -> MythraResult<String> {
        let mut results = String::new();
        match env::current_exe() {
            Ok(exe_path) => {
                let mut val_map = String::from("");
                    for (key, val) in params {
                      val_map = val_map + &(format!("{}={}", key, val))[..]
                 }
                let concat_url = url.to_owned() + &val_map[..];
                let (mut file, contents) = create_or_retrieve(concat_url, exe_path);
                // if file is empty then cache does not exist
                // then retrieve directly using reqwest
                if (contents.as_str()).eq("") || 
                    (contents.as_str()).eq("error code: 1020") ||
                    (contents.as_str()).eq("({\"response\":null});"){
                    let res = reqwest::Client::new()
                        .post(url)
                        .form(&params).send()
                        .await?.text().await?;
                    file.write_all((res.as_str()).as_bytes())?;
                    debug!("Retrieving {} [POST] data from web", url);
                    results = res;
                } else {
                    debug!("Retrieving {} [POST] data from cache", url);
                    results = contents;
                }
            }
            Err(e) => {
                format!("failed to get current exe path: {}", e);
            }
        };
        Ok(results)

    }

    pub async fn js_post(
        url: &String,
        form_selector: &str,
        params: &HashMap<&str, &str>,
    ) -> String {
        let new_url = url.clone();
        fantoccini_form(&new_url, form_selector, params)
            .await
            .ok()
            .unwrap()
    }

}
