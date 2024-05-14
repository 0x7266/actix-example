use std::io::Result;

use actix_web::{web, App, HttpServer, Responder};

#[actix_web::get("/")]
async fn index() -> impl Responder {
    format!("Hello, World!")
}

#[actix_web::get("/greet/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello, {}", name)
}

#[actix_web::main]
async fn main() -> Result<()> {
    println!("Starting server on port 3333");
    HttpServer::new(|| App::new().service(index).service(greet))
        .bind(("localhost", 3333))?
        .workers(2)
        .run()
        .await
}
