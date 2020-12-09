mod engines;
mod types;
mod utils;
mod download;
mod api;
use clap::{App, load_yaml};
use log::{info, error};

#[actix_web::main]
async fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();
    match matches.subcommand() {
        Some(("clear-cache", cache_matches)) => {
            // Clear cache
            // Start API server on port
            let verbosity = cache_matches.value_of("verbose")
                .unwrap();
            utils::configure_log(verbosity);
            utils::clear_cache();
        },

        Some(("search", search_matches)) => {
            // Search on CLI
            let verbosity = search_matches.value_of("verbose")
                .unwrap();
            utils::configure_log(verbosity);
            let engine = search_matches.value_of("engine")
                .unwrap();
            let query = search_matches.value_of("query")
                .unwrap();
            engines::cli(engine, query).await;
        },

        Some(("api", api_matches)) => {
            // Start API server on port
            let verbosity = api_matches.value_of("verbose")
                .unwrap();
            utils::configure_log(verbosity);
            let port = api_matches.value_of("port")
                .unwrap();
            info!("Running API on {:?}", port);
            let server = api::api(port).await;
            match server {
                Err(_) => info!("Error starting server"),
                Ok(_) => info!("Exiting..."),
            }
        },

        _ => error!("Select a valid subcommand"),

    }
}
