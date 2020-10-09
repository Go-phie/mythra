use scraper::{Selector, ElementRef};

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
