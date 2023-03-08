use actix_web::{get, web, web::Data, App, HttpServer, Responder};
use log::{info, warn};
use dotenv::dotenv;
use crate::config::db;
use mongodb::Client;
use std::env;
use serde::Serialize;

mod api;
mod models;
mod config;
mod repository;

#[derive(Serialize)]
struct ErrorStatus {
    status: String,
    msg: String
}

#[get("/events")]
async fn events(client: Data<Client>) -> impl Responder {
    let obj = ErrorStatus {
        status: "error".to_string(),
        msg: "Can't find any events.".to_string()
    };

    return web::Json(obj);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    println!("⚙️ LynixAPI is Initializing...");

    // Load Env File
    dotenv().ok();

    // Initialize MongoDB Connection
    let client = db::init().await;

    // Get Host Settings
    let server_host = match env::var("SERVER_HOST") {
        Ok(v) => v.to_string(),
        Err(_) => "127.0.0.1".to_string(),
    };

    let server_port: u16 = match env::var("SERVER_PORT") {
        Ok(v) => v.parse().unwrap_or(8080),
        Err(_) => 8080,
    };

    println!("✨ LynixAPI Started {}:{}", server_host, server_port);
    HttpServer::new(move || {
        App::new()
            .configure(api::index_api::init)
            .configure(api::event_api::init) // Events Service System
            .app_data(Data::new(client.clone()))
    })
        .bind(("127.0.0.1", 8557))?
        .run()
        .await
}