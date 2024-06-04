use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    io::Result,
    sync::{Arc, Mutex},
};

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

#[actix_web::get("/users")]
async fn get_users(db: web::Data<UserDb>) -> impl Responder {
    let mut users = Vec::new();
    let db = db.lock().unwrap();
    for k in db.keys() {
        match db.get(k) {
            Some(user) => users.push(user),
            None => println!("Error while getting user #{}", k),
        }
    }
    HttpResponse::Ok().json(users)
}

#[actix_web::post("users/new")]
async fn create_user(user_data: web::Json<User>, db: web::Data<UserDb>) -> impl Responder {
    let mut db = db.lock().unwrap();
    let new_id = db.keys().max().unwrap_or(&0) + 1;
    let name = user_data.name.clone();
    db.insert(new_id, user_data.into_inner());
    HttpResponse::Created().json(User { name })
}

#[actix_web::main]
async fn main() -> Result<()> {
    println!("Starting server on port 3333");

    let user_db: UserDb = Arc::new(Mutex::new(HashMap::<u32, User>::new()));
    HttpServer::new(move || {
        let app_data = web::Data::new(user_db.clone());
        App::new()
            .app_data(app_data)
            .service(index)
            .service(greet)
            .service(get_users)
            .service(create_user)
    })
    .bind(("localhost", 3333))?
    .workers(2)
    .run()
    .await
}
