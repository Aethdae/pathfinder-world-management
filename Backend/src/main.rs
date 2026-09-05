use actix_web::{App, Error, HttpServer, Responder, get, web};
use serde::Serialize;


#[derive(Serialize)]
struct MyObj{
    name: String
}

#[get("/a/{name}")]
async fn index(name: web::Path<String>) -> Result<impl Responder, Error> {
    let obj = MyObj {
        name: name.to_string(),
    };
    Ok(web::Json(obj))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    HttpServer::new(|| App::new().service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}