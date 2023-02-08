// mod model;
mod routes;
// use crate::model::model::{SignupInfo,LoginInfo};
use crate::routes::routes::{login, register};
use actix_web::{get,post, web, App, HttpServer,HttpRequest, Responder};
use std::io;

async fn health_check(req: HttpRequest) -> impl Responder {
    "All Ok"
}

#[tokio::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
                .route("/health-check", web::get().to(health_check))
                .route("/login", web::post().to(login))
                .route("/register", web::post().to(register))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
    // println!("Hello, world!");
}
