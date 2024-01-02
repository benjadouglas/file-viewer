#[allow(unused)]
use axum::{routing::get, Router, response::Json};
use std::env;
use std::net::SocketAddr;
use std::path::Path;
use tower_http::cors::CorsLayer;
use serde::{Deserialize, Serialize};
use serde_json::to_string;

#[derive(Serialize, Deserialize)]
pub struct my_file {
    name: String,
}

#[derive(Serialize, Deserialize)]
pub struct my_dir{
    path: String,
    files: Vec<my_file>,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        //allow frontend to fetch data
        .layer(CorsLayer::very_permissive());
    let adress = SocketAddr::from(([127, 0, 0, 1], 8080));
    axum::Server::bind(&adress)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index() -> Json<my_dir> {
    let arr = serial_killer("/benja/Code/web/file_viewer/frontend/");
    Json(arr)
}

fn get_dirs(s: &str) -> Vec<String> {
    let home_dir = env::var("HOME").expect("HOME environment variable not set");
    let dir_path = format!("{}{}", home_dir, s);
    let dir = Path::new(&dir_path);
    let mut arr: Vec<String> = Vec::new();
    for entry in dir.read_dir().expect("Couldn't read dir") {
        match entry {
            Ok(entry) => {
                let path = entry.path();
                arr.push(path.to_str().unwrap().to_string());
            }
            Err(e) => println!("Error reading directory: {}", e),
        }
    }
    return arr;
}

fn files(s: &str) -> Vec<String> {
    let home_dir = env::var("HOME").expect("HOME environment variable not set");
    let dir_path = format!("{}{}", home_dir, s);
    let dir = Path::new(&dir_path);
    let mut arr: Vec<String> = Vec::new();
    for entry in dir.read_dir().expect("Couldn't read dir") {
        match entry {
            Ok(entry) => {
                let path = entry.file_name();
                arr.push(path.to_str().unwrap().to_string());
            }
            Err(e) => println!("Error reading directory: {}", e),
        }
    }
    return arr;
}

fn serial_killer(s: &str) -> my_dir{
    let _files = files(s);
    let mut  arr: Vec<my_file> = Vec::new();
    for  file in _files.iter() {
        arr.push(my_file{name: file.to_string()});
    }
    let obj = my_dir{path: s.to_string(), files: arr};
    return obj
}
