use actix_web::{App, Error, HttpResponse, HttpServer, Responder, get, post, web};
use serde::{Deserialize, Serialize};
use actix_cors::Cors;
use std::{collections::HashMap, vec};
use serde_json::Value;
use rusqlite::{Connection, Result};
use env_file_reader::read_file;
use rust_fetch::{Fetch, FetchConfig, FetchOptions, FetchResponse};

#[derive(Deserialize, Serialize, Debug)]
struct Data{
    data: Value
}

#[derive(Deserialize, Serialize, Debug)]
struct SqlReturn{
    result: Vec<ResArray>,
    errors: Vec<String>,
    messages: Vec<String>,
    success: bool
}

#[derive(Deserialize, Serialize, Debug)]
struct ResArray{
    results: Vec<Results>,
    success: bool,
    meta: Meta
}

#[derive(Deserialize, Serialize, Debug)]
struct Results{
    id: i32,
    name: String,
    data: String
}

#[derive(Deserialize, Serialize, Debug)]
struct Timings{
    sql_duration_ms: f32
}

#[derive(Deserialize, Serialize, Debug)]
struct Meta{
    served_by: String,
    served_by_region: String,
    served_by_colo: String,
    served_by_primary: bool,
    timings: Timings,
    duration: f32,
    changes: i32,
    last_row_id: i32,
    changed_db: bool,
    size_after: i32,
    rows_read: i32,
    rows_written: i32,
    total_attempts: i32
}

#[derive(Deserialize, Serialize, Debug)]
struct SqlSend{
    sql: String,
    params: Vec<String>
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
async fn get_name(name: web::Path<String>) -> Result<impl Responder, Error> {
    let env_vars = read_file("./.env")?;
    let cloudflare_api_token = &env_vars["CLOUDFLARE_API_TOKEN"];
    let cloudflare_account_id = &env_vars["CLOUDFLARE_ACCOUNT_ID"];
    let d1_database_uuid = &env_vars["D1_DATABASE_UUID"];

    let mut headers = HashMap::new();
    headers.insert("Authorization".to_string(), format!("Bearer {cloudflare_api_token}"));

    let fetch_config = FetchConfig{
        timeout_ms: Some(2000u64),
        headers: Some(headers),
        content_type: rust_fetch::ContentType::Json,
        accept: rust_fetch::ContentType::Json
    };

    let url = format!("https://api.cloudflare.com/client/v4/accounts/{cloudflare_account_id}/d1/database/{d1_database_uuid}");
    let client = Fetch::new(&url, Some(fetch_config)).unwrap();

    let query = SqlSend{
        sql: "SELECT * FROM world WHERE name = ?;".to_string(),
        params: vec![name.to_string()]
    };

    let mut headers = HashMap::new();
    headers.insert("Authorization".to_string(), format!("Bearer {cloudflare_api_token}"));
    
    let res: FetchResponse<SqlReturn> = client.post("/query", Some(query), Some(FetchOptions{
        headers: Some(headers),
        content_type: Some(rust_fetch::ContentType::Json),
        ..Default::default()
    })).await.unwrap();

    // let connection = Connection::open("./pf.db").unwrap();
    // let mut statement = connection.prepare("SELECT name, data 
    // FROM world WHERE name = ?1").expect("Error preparing statement for SQL.");
    // let iter = statement.query_row([name.to_string()], |row| {
    //     Ok(Place{
    //         id: 1,
    //         name: row.get(0)?,
    //         data: serde_json::Value::String(row.get(1)?)
    //     })
    // }).expect("Error parsing data from table.");

    Ok(web::Json(res.body))
}

#[get("/fetchRand")]
async fn get_rand() -> Result<impl Responder, Error> {
    let env_vars = read_file("./.env")?;
    let cloudflare_api_token = &env_vars["CLOUDFLARE_API_TOKEN"];
    let cloudflare_account_id = &env_vars["CLOUDFLARE_ACCOUNT_ID"];
    let d1_database_uuid = &env_vars["D1_DATABASE_UUID"];

    let mut headers = HashMap::new();
    headers.insert("Authorization".to_string(), format!("Bearer {cloudflare_api_token}"));
    
    let fetch_config = FetchConfig{
        timeout_ms: Some(2000u64),
        headers: Some(headers),
        content_type: rust_fetch::ContentType::Json,
        accept: rust_fetch::ContentType::Json
    };

    let url = format!("https://api.cloudflare.com/client/v4/accounts/{cloudflare_account_id}/d1/database/{d1_database_uuid}");
    let client = Fetch::new(&url, Some(fetch_config)).unwrap();

    let query = SqlSend{
        sql: "SELECT name FROM world;".to_string(),
        params: vec!["".to_string()]
    };

    let mut headers = HashMap::new();
    headers.insert("Authorization".to_string(), format!("Bearer {cloudflare_api_token}"));

    let res: FetchResponse<SqlReturn> = client.post("/query", Some(query), Some(FetchOptions{
    headers: Some(headers),
    content_type: Some(rust_fetch::ContentType::Json),
    ..Default::default()
    })).await
    .unwrap();

    Ok(web::Json(res.body))
}

#[post("/{name}")]
async fn add_new(name: web::Path<String>, data: web::Json<Data>) -> Result<impl Responder, Error> {
    let env_vars = read_file("./.env")?;
    let cloudflare_api_token = &env_vars["CLOUDFLARE_API_TOKEN"];
    let cloudflare_account_id = &env_vars["CLOUDFLARE_ACCOUNT_ID"];
    let d1_database_uuid = &env_vars["D1_DATABASE_UUID"];

    println!("{}, \n{}, \n{}", cloudflare_account_id, cloudflare_api_token, d1_database_uuid);

    let connection = Connection::open("./pf.db").unwrap();
    let data_str = serde_json::to_string(&data.data).expect("Failed to unpack Json");

    if let Err(err) = connection.execute("INSERT INTO world(name, data)
    VALUES(?1, ?2)
    ", (&name.to_string(), &data_str.to_string()), )
    {
        println!("Error occurred: {}", err);
    }
    
    let data_ret = Data{
        data: serde_json::Value::String(data_str)
    };
    Ok(HttpResponse::Ok().json(data_ret))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    init_db().await;

    HttpServer::new(|| {
        let cors = Cors::default()
        .allowed_origin("https://pathfinder.aethdae.com")
        .allowed_methods(vec!["GET", "POST"])
        .allow_any_header()
        .max_age(3600);

        App::new()
        .service(get_name)
        .service(health)
        .service(add_new)
        .service(get_rand)
        .wrap(cors)})
        .bind(("0.0.0.0", 10000))?
        .run()
        .await
}

async fn init_db() {
    println!("Setting up DB..");
    let connection = Connection::open("./pf.db").unwrap();
    match connection.execute("
    CREATE TABLE IF NOT EXISTS world(
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    data TEXT);", (),){
        Ok(_) => println!("Ok! Created DB and world table."),
        Err(err) => println!("Error occurred: {}", err)
    };
}