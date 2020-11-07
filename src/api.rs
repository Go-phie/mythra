use actix_web::{http::StatusCode, web, App, HttpServer, HttpResponse};
use crate::types::MusicRequest;
use crate::types::EngineTraits;
use crate::engines::mp3red;
use std::env;

async fn index(web::Query(info): web::Query<MusicRequest>) -> HttpResponse {
    format!("Request for client with engine={} and query={}!", info.engine, info.query);
        let query = info.query.clone();
        let engine = info.engine.clone();
        let engine_match = engine.as_str();
        match engine_match {
            "mp3red" => {
                let e = mp3red::MP3Red;
                let res = EngineTraits::search(&e, query);
                HttpResponse::Ok().json(res.unwrap())
            },
            _ => {
                println!("Engine {} is unsupported", engine_match);
                HttpResponse::new(StatusCode::NOT_FOUND)
            },
        }
}

pub async fn server(port: &str) -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
//    env_logger::init();
    let address: &str = &(format!("127.0.0.1:{}", port))[..];
    HttpServer::new(|| 
                    App::new().service(
                        web::resource("/search")
                        .route(
                            web::get().to(index)
                            )
                        )
                   )
        .bind(address)
        .unwrap()
        .run()
        .await
}
