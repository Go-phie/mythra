use crate::engines::get_engine;
use crate::types::{
    MusicRequest
};

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
    let engine = get_engine(engine_match);
    match engine {
        Ok(actual) => {
            let res = actual.search(query).await.ok();
            HttpResponse::Ok().json(res.unwrap())
        },
        Err(_) => {
            error!("Error {} is unsupported", engine_match);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_rt::test]
    async fn test_api_with_fake_engine_returns_not_found() {
        let query: web::Query<MusicRequest> = web::Query::from_query("engine=fake&query=real").unwrap();
        assert_eq!(search(query).await.status(), StatusCode::NOT_FOUND);
    }
}
