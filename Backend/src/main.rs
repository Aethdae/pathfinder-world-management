use actix_web::{App, Error, HttpServer, Responder, get, post, web::{self}};
use serde::Serialize;
use serde_json::Value;
use sqlite;


#[derive(Serialize)]
struct Data{
    name: String,
    data: Value
}

#[derive(Serialize)]
struct Healthy{
    health: String
}

#[get("/")]
async fn health() -> Result<impl Responder, Error> {
    let health = Healthy {
        health: "Ok".to_string()
    };
    Ok(web::Json(health))
}

#[get("/{name}")]
async fn index(name: web::Path<String>) -> Result<impl Responder, Error> {
    let connection   = sqlite::open("pf.db").unwrap();
    let exe = connection.execute("
    CREATE TABLE IF NOT EXISTS world(
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    data TEXT);
    INSERT INTO world(name, data)
    VALUES('test', '{\"test\":\"test\"}')
    ").unwrap();
    println!("{:?}, {}", exe, name);
    Ok("Ok")
}

#[post("/{name}")]
async fn add_new(name: web::Path<String>, data: web::Data<String>) -> Result<impl Responder, Error> {
    let data = Data{
        name: name.to_string(),
        data: serde_json::from_str(&data)?
    };
    Ok(web::Json(data))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    HttpServer::new(|| App::new().service(index).service(health))
        .bind(("127.0.0.1", 3000))?
        .run()
        .await
}