mod engines;
use clap::{App, load_yaml};

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();
    println!("{:?}", matches);
    let engine = engines::redthreemp3::RedThreeMP3::new();
}
