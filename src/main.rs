use actix_web::{
    middleware, web, App, HttpResponse, HttpServer,
};
#[macro_use]
extern crate serde_json;

macro_rules! get_app {
    () => {
        App::new()
            .service(web::resource("/").route(web::get().to(index)))
    };
}

async fn index() -> HttpResponse {
    HttpResponse::Ok().json(json!({ "say": "Hi" }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(move || {
        get_app!()
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
        let mut app = test::init_service(
            get_app!()
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
        let mut app = test::init_service(
            get_app!()
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
}