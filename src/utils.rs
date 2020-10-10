use scraper::{Selector, ElementRef};

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
            return String::from(
                tag.value().attr(others).unwrap()
                )
        },
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
                let full_dir_path = format!("{}/cache", parent);
                let full_path = format!("{}/cache/{}", parent, hashed_url);
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
