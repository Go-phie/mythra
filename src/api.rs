use actix_web::{http::StatusCode, web, App, HttpServer, HttpResponse};
use actix_web::{middleware::Logger};
use crate::types::MusicRequest;
use crate::engines::mp3red;
use log::{error, debug};

async fn index(web::Query(info): web::Query<MusicRequest>) -> HttpResponse {
    debug!("Request for client with engine={} and query={}!", info.engine, info.query);
        let query = info.query.clone();
        let engine = info.engine.clone();
        let engine_match = engine.as_str();
        match engine_match {
            "mp3red" => {
                let e = mp3red::MP3Red{};
                let res = e.search(query).await.ok();
                HttpResponse::Ok().json(res.unwrap())
            },
            _ => {
                error!("Engine {} is unsupported", engine_match);
                HttpResponse::new(StatusCode::NOT_FOUND)
            },
        }
}

pub async fn api(port: &str) -> std::io::Result<()> {
    let address: &str = &(format!("127.0.0.1:{}", port))[..];
    HttpServer::new(|| 
                    App::new()
                    .wrap(Logger::default())
                    .wrap(Logger::new("%a %{User-Agent}i"))
                    .service(
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
