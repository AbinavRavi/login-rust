mod routes;
use crate::routes::routes::{login, register, forget_password, refresh, reset_password, logout, verify};
use actix_web::{web, App, HttpServer,HttpRequest, Responder};
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
                .route("/forget_password",web::get().to(forget_password))
                .route("/refresh", web::get().to(refresh))
                .route("/logout", web::post().to(logout))
                .route("/verify", web::post().to(verify))
                .route("/reset_password", web::post().to(reset_password))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
    // println!("Hello, world!");
}
