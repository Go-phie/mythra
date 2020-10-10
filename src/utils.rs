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

    pub fn get(url: &String) {
        match env::current_exe() {
            Ok(exe_path) => {
                let path = Path::new(exe_path.to_str().unwrap());
                let parent = path.parent();
            },
            Err(e) => println!("failed to get current exe path: {}", e),
        };
    }
}
