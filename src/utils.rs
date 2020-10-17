use scraper::{Selector, ElementRef};
use log::debug;
use log::{Record, Level, Metadata};
use crate::types::Music;
use cursive::views::{Dialog, SelectView, TextView};
use cursive::align::HAlign;
use cursive::Cursive;
use cursive::view::{Scrollable, Resizable};

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


pub struct SimpleLogger;
impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

            

// Removes cache directory
pub fn clear_cache(){
    use std::fs;
    use std::env;
    use std::path::Path;
    use log::info;
    match env::current_exe() {
        Ok(current_exe) => {
            let path: &Path = Path::new(current_exe.to_str().unwrap());
            let parent: &str = path.parent().unwrap().to_str().unwrap();
            let full_dir_path = format!("{}/{}", parent, crate::utils::CACHE_NAME);
            fs::remove_dir_all(full_dir_path).unwrap();
            info!("Mythra cache cleared!");
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
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::fs::{OpenOptions, create_dir_all};
use std::io::{Read, Write};
use log::debug;

    pub fn get(url: &String) -> Result<String, Box<dyn std::error::Error>> {
    let mut results = String::new();
    match env::current_exe() {
            Ok(exe_path) => {
                let path: &Path = Path::new(exe_path.to_str().unwrap());
                let parent: &str = path.parent().unwrap().to_str().unwrap();
                // hash url 
                let mut hasher = DefaultHasher::new();
                url.hash(&mut hasher);
                let hashed_url: &str = &(hasher.finish().to_string())[..];
                let full_dir_path = format!("{}/{}", parent, crate::utils::CACHE_NAME);
                let full_path = format!("{}/{}",full_dir_path, hashed_url);
                // create all parent directories necessary
                create_dir_all(full_dir_path)?;
                let mut file = OpenOptions::new()
                    .write(true).read(true)
                    .create(true).open(full_path)
                    .unwrap();
                // read file contents to String
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
                // if file is empty then cache does not exist
                // then retrieve directly using reqwest
                if (&contents[..]).eq("") {
                    let res = reqwest::blocking::get(url)?
                        .text()?;
                    file.write_all((&res[..]).as_bytes())?;
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
}

pub fn render_select_music(songs:&'static mut Vec<Music>, title: &str){
    let mut select = SelectView::new()
        .h_align(HAlign::Center)
        .autojump();
    for song in songs.iter_mut() {
        let title = &(song.title)[..];
        select.add_item(title, &mut song);
    }
    select.set_on_submit(download_song);
    let mut siv = cursive::default();
    // Let's add a ResizedView to keep the list at a reasonable size
    // (it can scroll anyway).
    siv.add_layer(
        Dialog::around(select.scrollable().fixed_size((20, 10)))
        .title(title),
        );
}

pub fn download_song(siv: &mut Cursive, song: &Music){
    siv.pop_layer();
    let text = format!("Downloading {} ...", song.title);
    siv.add_layer(
        Dialog::around(TextView::new(text)).button("Quit", |s| s.quit()),
        );
}