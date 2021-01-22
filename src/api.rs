use crate::engines::mp3s;
use crate::engines::myfreemp3;
use crate::types::MusicRequest;

use actix_web::{http::StatusCode, web, App, HttpResponse, HttpServer};
//use actix_cors::Cors;
use actix_web::middleware::Logger;
use log::{debug, error};

async fn search(web::Query(info): web::Query<MusicRequest>) -> HttpResponse {
    debug!(
        "Request for client with engine={} and query={}!",
        info.engine, info.query
    );
    let query = info.query.clone();
    let engine = info.engine.clone();
    let engine_match = engine.as_str();
    match engine_match {
        "mp3s" => {
            let e = mp3s::MP3S {};
            let res = e.search(query).await.ok();
            HttpResponse::Ok().json(res.unwrap())
        }
        "myfreemp3" => {
            let e = myfreemp3::MyFreeMP3 {};
            let res = e.search(query).await.ok();
            HttpResponse::Ok().json(res.unwrap())
        }
        _ => {
            error!("Engine {} is unsupported", engine_match);
            HttpResponse::new(StatusCode::NOT_FOUND)
        }
    }
}

pub async fn api(port: &str) -> std::io::Result<()> {
    let address: &str = &(format!("0.0.0.0:{}", port))[..];
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(web::resource("/search").route(web::get().to(search)))
    })
    .bind(address)
    .unwrap()
    .run()
    .await
}
