mod engines;
mod types;
mod utils;
mod api;
use clap::{App, load_yaml};

#[actix_web::main]
async fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();
    match matches.subcommand() {
        Some(("clear-cache", _)) => {
        // Clear cache
            utils::clear_cache();
        },

        Some(("search", search_matches)) => {
        // Search on CLI
            let engine = search_matches.value_of("engine")
                .unwrap();
            let query = search_matches.value_of("query")
                .unwrap();
            engines::search_all(engine, query).await;
        },

        Some(("api", api_matches)) => {
        // Start API server on port
            let port = api_matches.value_of("port")
                .unwrap();
            println!("Running API on {:?}", port);
            let server = api::server(port).await;
            match server {
                Err(_) => println!("Error starting server"),
                Ok(_) => println!("Exiting..."),
            }
        },

        _ => println!("Select a valid subcommand"),

    }
}
