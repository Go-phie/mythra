mod engines;
mod types;
use clap::{App, load_yaml};

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();
    println!("{:?}", matches);
    engines::search_all();
}
