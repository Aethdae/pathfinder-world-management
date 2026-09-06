use actix_web::{App, Error, HttpServer, Responder, get, web::{self}};
use serde::Serialize;
use serde_json::Value;


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
    let str = r#"
    {
    "test": "test1", 
    "test2": "test3"
    }
    "#;
    let v: Value = serde_json::from_str(str)?;
    let obj = Data {
        name: name.to_string(),
        data: v
    };
    Ok(web::Json(obj))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    HttpServer::new(|| App::new().service(index).service(health))
        .bind(("127.0.0.1", 3000))?
        .run()
        .await
}