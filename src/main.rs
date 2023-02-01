mod model;
use crate::model::model::{SignupInfo,LoginInfo};
use actix_web::{get, web, App, HttpServer, Responder};
use std::io;

async fn index() -> impl Responder {
    "All Ok"
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            // prefixes all resources and routes attached to it...
            web::scope("/users")
                // ...so this handles requests for `GET /app/index.html`
                .route("/health-check", web::get().to(index)),
        )
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
    // println!("Hello, world!");
}
