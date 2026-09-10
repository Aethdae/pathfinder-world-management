use actix_web::{App, Error, HttpResponse, HttpServer, Responder, get, post, web};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use rusqlite::{Connection, Result};


#[derive(Deserialize, Serialize, Debug)]
struct Data{
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
async fn index(_name: web::Path<String>) -> Result<impl Responder, Error> {
    let connection = Connection::open("./pf.db").unwrap();
    match connection.execute("
    CREATE TABLE IF NOT EXISTS world(
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    data TEXT);", (),){
        Ok(_) => println!("Ok! Created world table."),
        Err(err) => println!("Error occurred: {}", err)
    };
    Ok("Ok")
}

#[post("/{name}")]
async fn add_new(name: web::Path<String>, data: web::Json<Data>) -> Result<impl Responder, Error> {
    let connection = Connection::open("./pf.db").unwrap();
    println!("{}, {:?}", name, data);
    let data_str = serde_json::to_string(&data.data).expect("Failed to unpack Json");
    match connection.execute("
    CREATE TABLE IF NOT EXISTS world(
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    data TEXT);", (),){
        Ok(_) => println!("Ok! Created world table."),
        Err(err) => println!("Error occurred: {}", err)
    };
    match connection.execute("INSERT INTO world(name, data)
    VALUES(?1, ?2)
    ", (&name.to_string(), &data_str.to_string()), ){
        Ok(amount) => println!("Ok! Changed {} rows.", amount),
        Err(err) => println!("Error occurred: {}", err)        
    };
    
    let data_ret = Data{
        data: serde_json::Value::String(data_str)
    };
    Ok(HttpResponse::Ok().json(data_ret))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    HttpServer::new(|| App::new()
        .service(index)
        .service(health)
        .service(add_new))
        .bind(("127.0.0.1", 3000))?
        .run()
        .await
}