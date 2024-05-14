use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::io::Result;
use std::sync::Arc;
use std::sync::Mutex;

use actix_web::{web, App, HttpServer, Responder};

type UserDb = Arc<Mutex<HashMap<u32, User>>>;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    name: String,
}

#[actix_web::get("/")]
async fn index() -> impl Responder {
    format!("Hello, World!")
}

#[actix_web::get("/greet/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    web::Json(User {
        name: name.to_string(),
    })
}

#[actix_web::main]
async fn main() -> Result<()> {
    println!("Starting server on port 3333");

    let user_db: UserDb = Arc::new(Mutex::new(HashMap::<u32, User>::new()));
    HttpServer::new(move || {
        let app_data = web::Data::new(user_db.clone());
        App::new().app_data(app_data).service(index).service(greet)
    })
    .bind(("localhost", 3333))?
    .workers(2)
    .run()
    .await
}
