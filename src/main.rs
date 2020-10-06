use actix_web::{
    middleware, web, App, HttpResponse, HttpServer,
};
#[macro_use]
extern crate serde_json;

use serde::{Deserialize, Serialize};

mod printer;

use printer::Printer;
use std::sync::{Arc, RwLock};

#[macro_use]
extern crate struct2swagger_derive;
#[macro_use]
extern crate struct2swagger;

use struct2swagger::{JsonSchemaDefinition, QueryDefinition, swagger_object::SwaggerObject};


macro_rules! get_app {
    ($printer: ident, $swagger: ident) => {
        App::new()
            .data($printer.clone())
            .data($swagger.clone())
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/doc").route(web::get().to(doc)))
    };
}

fn get_openapi_spec() -> SwaggerObject {
    let mut swagger_object = SwaggerObject::new(
      "my-rust-webserver", // title
      "1.0.0" // version
    );
  
    swagger_add_router!(
        swagger_object, // obj
        "GET", // method
        "/", // path
        HelloWorldQuery, // query parameters
        200, // expected status code
        "say", //  description
        HelloWorldResponse // struct in output
    );

    swagger_object
}

#[derive(Deserialize, Swagger)]
struct HelloWorldQuery {
    who: Option<String>
}

#[derive(Serialize, Swagger)]
struct HelloWorldResponse {
    say: String
}

async fn index(query: web::Query<HelloWorldQuery>, printer: web::Data<Arc<RwLock<Printer>>>) -> HttpResponse {
    let printer = printer.into_inner();
    let printer = printer.read().unwrap();

    let query = query.into_inner();
    let who = query.who.unwrap_or("World".to_owned());

    let say = printer.say(who);

    let response = HelloWorldResponse { say };
    HttpResponse::Ok().json(response)
}

async fn doc(
    swagger: web::Data<SwaggerObject>
) -> HttpResponse {
    let swagger = &*swagger.into_inner();
    HttpResponse::Ok().json(swagger)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let printer = Printer {};
    let printer = Arc::new(RwLock::new(printer));

    let swagger = get_openapi_spec();

    HttpServer::new(move || {
        get_app!(printer, swagger)
            // enable logger
            .wrap(middleware::Logger::default())
    })
    .bind("0.0.0.0:8081")?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http, test, web, App, Error, dev::Service};

    macro_rules! get_test_app {
        () => {
            {
                let printer = Printer {};
                let printer = Arc::new(RwLock::new(printer));
                let swagger = get_openapi_spec();

                let app = test::init_service(
                    get_app!(printer, swagger)
                )
                .await;

                app
            }
        };
    }

    #[actix_rt::test]
    async fn test_index_default() -> Result<(), Error> {
        let mut app = get_test_app!();

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
        let mut app = get_test_app!();

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
        let mut app = get_test_app!();

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

    #[actix_rt::test]
    async fn test_doc() -> Result<(), Error> {
        let mut app = get_test_app!();

        let req = test::TestRequest::get()
            .uri("/doc")
            .to_request();
        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), http::StatusCode::OK);

        Ok(())
    }
}