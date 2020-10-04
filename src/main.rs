use actix_web::{
    middleware, web, App, HttpResponse, HttpServer,
};
#[macro_use]
extern crate serde_json;

use serde::{Deserialize};

mod printer;

use printer::Printer;
use std::sync::{Arc, RwLock};

macro_rules! get_app {
    ($printer: ident) => {
        App::new()
            .data($printer.clone())
            .service(web::resource("/").route(web::get().to(index)))
    };
}

#[derive(Deserialize)]
struct HelloWorldQuery {
    who: Option<String>
}

async fn index(query: web::Query<HelloWorldQuery>, printer: web::Data<Arc<RwLock<Printer>>>) -> HttpResponse {
    let printer = printer.into_inner();
    let printer = printer.read().unwrap();

    let query = query.into_inner();
    let who = query.who.unwrap_or("World".to_owned());

    let say = printer.say(who);
    HttpResponse::Ok().json(json!({ "say": say }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let printer = Printer {};
    let printer = Arc::new(RwLock::new(printer));

    HttpServer::new(move || {
        get_app!(printer)
            // enable logger
            .wrap(middleware::Logger::default())
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http, test, web, App, Error, dev::Service};

    #[actix_rt::test]
    async fn test_index_default() -> Result<(), Error> {
        let printer = Printer {};
        let printer = Arc::new(RwLock::new(printer));

        let mut app = test::init_service(
            get_app!(printer)
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/")
            .to_request();
        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), http::StatusCode::OK);

        let response_body = match resp.response().body().as_ref() {
            Some(actix_web::body::Body::Bytes(bytes)) => bytes,
            _ => panic!("Response error"),
        };

        assert_eq!(response_body, r##"{"say":"Hi World!"}"##);

        Ok(())
    }

    #[actix_rt::test]
    async fn test_index_with_query_params() -> Result<(), Error> {
        let printer = Printer {};
        let printer = Arc::new(RwLock::new(printer));

        let mut app = test::init_service(
            get_app!(printer)
        )
        .await;
        let req = test::TestRequest::get()
            .uri("/?who=Tom")
            .to_request();
        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), http::StatusCode::OK);

        let response_body = match resp.response().body().as_ref() {
            Some(actix_web::body::Body::Bytes(bytes)) => bytes,
            _ => panic!("Response error"),
        };

        assert_eq!(response_body, r##"{"say":"Hi Tom!"}"##);

        Ok(())
    }

    #[actix_rt::test]
    async fn test_index_with_empty_query_params() -> Result<(), Error> {
        let printer = Printer {};
        let printer = Arc::new(RwLock::new(printer));

        let mut app = test::init_service(
            get_app!(printer)
        )
        .await;
        let req = test::TestRequest::get()
            .uri("/?who=")
            .to_request();
        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), http::StatusCode::OK);

        let response_body = match resp.response().body().as_ref() {
            Some(actix_web::body::Body::Bytes(bytes)) => bytes,
            _ => panic!("Response error"),
        };

        assert_eq!(response_body, r##"{"say":"Hi!"}"##);

        Ok(())
    }
}