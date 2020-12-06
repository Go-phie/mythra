use scraper::{Selector, ElementRef};
use log::debug;
use crate::types::Music;
use cursive::views::{Dialog, SelectView, TextView};
use cursive::align::HAlign;
use cursive::Cursive;
use cursive::view::{Scrollable, Resizable};
use env_logger::Builder;
use std::env;

pub static CACHE_NAME: &str = ".mythra-cache";
// Reduces the stress of repetitive extraction of elements
// as the raw scraper library is too verbose
pub fn extract_from_el(element:&ElementRef, selector:&str, attr:&str) -> String {
    let selector = Selector::parse(selector).unwrap();
    let tag = element.select(&selector).next().unwrap();
    match attr {
        "text" => {
            return tag.text().collect::<String>();    
        },
        others => 
        {
            let val = String::from(
                tag.value().attr(others).unwrap()
                );
            if others.eq("href") {
                debug!("Retrieved link {}", val)
            };
            return val
        },
    }
}
            

// Removes cache directory
pub fn clear_cache(){
    use std::fs;
    use std::path::Path;
    use log::info;
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

        },
        Err(_) => { () }
    }
}

// Wrapper around the reqwest module
// Retrieve web pages from cache if they exist
// else retrieve from url
pub mod cached_reqwest {
use std::env;
use std::path::Path;
use std::collections::{hash_map::DefaultHasher, HashMap};
use std::hash::{Hash, Hasher};
use std::fs::{OpenOptions, create_dir_all};
use std::io::{Read, Write};
use log::debug;
use fantoccini;
//std::fs::File, String
    
    pub fn create_or_retrieve(url: String, exe_path: std::path::PathBuf) -> (std::fs::File, String){
                let path: &Path = Path::new(exe_path.to_str().unwrap());
                let parent: &str = path.parent().unwrap().to_str().unwrap();
                // hash url 
                let mut hasher = DefaultHasher::new();
                url.hash(&mut hasher);
                let hashed_url: &str = &(hasher.finish().to_string())[..];
                let full_dir_path = format!("{}/{}", parent, crate::utils::CACHE_NAME);
                let full_path = format!("{}/{}",full_dir_path, hashed_url);
                // create all parent directories necessary
                create_dir_all(full_dir_path).ok().unwrap();
                let mut file = OpenOptions::new()
                    .write(true).read(true)
                    .create(true).open(full_path)
                    .unwrap();
                // read file contents to String
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
//                Ok((file, contents))
                (file, contents)
    }

    pub async fn gecko_form(url: &String, form_selector: &str, params: &HashMap<&str, &str>) -> Result<String, Box<dyn std::error::Error>> {
    let mut results = String::new();
    match env::current_exe() {
            Ok(exe_path) => {
                let mut val_map = String::from("");
                for (key, val) in params{
                    val_map = val_map + &(format!("{}={}", key,val))[..]
                }
                let concat_url = url.to_owned() + &val_map[..];
                let (mut file, contents) = create_or_retrieve(concat_url, exe_path);
                // if file is empty then cache does not exist
                // then retrieve directly using reqwest
                if (contents.as_str()).eq("") {
                    let mut c = fantoccini::Client::new("http://localhost:4444").await.expect("failed to connect to WebDriver");
                    c.goto(url).await.unwrap();
                    let mut form =  c.form(
                        fantoccini::Locator::Css(form_selector)
                        ).await.unwrap();
                    for (key, val) in params {
                        form.set_by_name(key, val).await.unwrap();
                    };
                    let mut res_client = form.submit().await.unwrap();
                    let res = res_client.source().await.unwrap();
                    file.write_all((res.as_str()).as_bytes())?;
                    c.close().await.unwrap();
                    debug!("Retrieving {} [POST] data from web (fantoccini)", url);
                    results = res;
                } else {
                    debug!("Retrieving {} [POST] data from cache (fantoccini)", url);
                    results = contents;
                }
            },
            Err(e) => {
                format!("failed to get current exe path: {}", e);
            },
        };
        return Ok(results)
    
    }

    pub async fn poster(url: &String, params: &HashMap<&str, &str>) -> Result<String, Box<dyn std::error::Error>> {
    let mut results = String::new();
    match env::current_exe() {
            Ok(exe_path) => {
                // hash url 
                let mut val_map = String::from("");
                for (key, val) in params{
                    val_map = val_map + &(format!("{}={}", key,val))[..]
                }
                let concat_url = url.to_owned() + &val_map[..];
                // if file is empty then cache does not exist
                // then retrieve directly using reqwest
                let (mut file, contents) = create_or_retrieve(concat_url, exe_path);
                if (contents.as_str()).eq("") {
                    let client = reqwest::Client::new();
                    let res = client.post(url)
                        .form(params)
                        .send()
                        .await
                        .unwrap()
                        .text()
                        .await
                        .unwrap();
                    file.write_all((res.as_str()).as_bytes())?;
                    debug!("Retrieving {} [POST] data from web", url);
                    results = res;
                } else {
                    debug!("Retrieving {} [POST] data from cache", url);
                    results = contents;
                }
            },
            Err(e) => {
                format!("failed to get current exe path: {}", e);
            },
        };
        return Ok(results)
    
    }

    pub async fn getter(url: &String) -> Result<String, Box<dyn std::error::Error>> {
    let mut results = String::new();
    match env::current_exe() {
            Ok(exe_path) => {
                let (mut file, contents) = create_or_retrieve(url.to_string(), exe_path);
                // if file is empty then cache does not exist
                // then retrieve directly using reqwest
                if (contents.as_str()).eq("") {
                    let res = reqwest::get(url)
                        .await
                        .unwrap()
                        .text()
                        .await
                        .unwrap();
                    file.write_all((res.as_str()).as_bytes())?;
                    debug!("Retrieving {} [GET] data from web", url);
                    results = res;
                } else {
                    debug!("Retrieving {} [GET] data from cache", url);
                    results = contents;
                }
            },
            Err(e) => {
                format!("failed to get current exe path: {}", e);
            },
        };
        return Ok(results)
    }

    pub async fn get(url: &String) -> String {
        let new_url = url.clone();
        getter(&new_url).await.ok().unwrap()
    }

    pub async fn post(url: &String, params: &HashMap<&str, &str>) -> String {
        let new_url = url.clone();
        poster(&new_url, params).await.ok().unwrap()
    }

    pub async fn submit_by_gecko(url: &String, form_selector: &str, params: &HashMap<&str, &str>) -> String {
        let new_url = url.clone();
        gecko_form(&new_url, form_selector, params).await.ok().unwrap()
    }
}

pub fn render_select_music(songs:Vec<Music>, title: &str){
    let mut select = SelectView::new()
        .h_align(HAlign::Center)
        .autojump();
    for song in songs {
        let title_copy = &song.title.clone();
        let title = title_copy.as_str();
        select.add_item(title, song);
    }
    select.set_on_submit(render_downloading_song);
    let mut siv = cursive::default();
    // Let's add a ResizedView to keep the 
    // list at a reasonable size (it can scroll anyway).
    siv.add_layer(
        Dialog::around(select.scrollable().fixed_size((50, 10)))
        .title(title),
        );
    siv.run();
}

pub fn render_downloading_song(siv: &mut Cursive, song: &Music){
//    replace previous view
//    siv.pop_layer();
    let text = format!("Downloading {} ...", song.title);
    siv.add_layer(
        Dialog::around(TextView::new(text)).button("Quit", |s| s.quit()),
        );
}


// configure logging
pub fn configure_log(level: &str){
    let logname: &str = "MYTHRA_LOG_FMT" ;
    // activate debugging for only actix_web and mythra
    let logfmt = format!("actix_web={},mythra={}", level, level);
    env::set_var(logname, logfmt);
    Builder::new()
        .parse_env(logname)
        .init();
}
