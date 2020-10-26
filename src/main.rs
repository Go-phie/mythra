mod engines;
mod types;
mod utils;
use clap::{App, load_yaml};
use log::LevelFilter;
use utils::SimpleLogger;

static LOGGER: SimpleLogger = SimpleLogger;

fn main() {
    let mut max_level = LevelFilter::Info;
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();
    match matches.subcommand() {
        Some(("clear-cache", _)) => utils::clear_cache(),
        Some(("search", search_matches)) => {
            if search_matches.is_present("verbose"){
                max_level = LevelFilter::Debug;
            }
            log::set_logger(&LOGGER)
                .map(|()| log::set_max_level(max_level))
                .unwrap();
            let engine = search_matches.value_of("engine")
                .unwrap();
            let query = search_matches.value_of("query")
                .unwrap();
            engines::search_all(engine, query);
        },
        _ => println!("Select a valid subcommand"),

    }
}
