#[allow(unused)]
use axum::{routing::get, Router};
use std::env;
use std::net::SocketAddr;
use std::path::Path;
use tower_http::cors::CorsLayer;
use serde::{Deserialize, Serialize};
use serde_json::to_string;

#[derive(Serialize, Deserialize)]
pub struct my_file {
    name: String,
    path: String,
    // file_type: String,
}

#[tokio::main]
async fn main() {
    serial_killer("/benja/Code/C++");
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

async fn index() -> String {
    let s = get_dir("/benja/Code/C++");
    let mut val = String::new();
    for i in s.iter() {
        val.push_str(i);
    }
    return val;
}

fn get_dir(s: &str) -> Vec<String> {
    let home_dir = env::var("HOME").expect("HOME environment variable not set");
    // let home_dir = "/Users/benjamindouglas";
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

fn serial_killer(s: &str) -> Vec<my_file>{
    let mut arr: Vec<my_file> = Vec::new();
    let dirs = get_dir(s);
    for (i, x) in dirs.iter().enumerate(){
        match x.rfind('/') {
            Some(index) => {
                let (_path, _name) = x.split_at(index+1);
                arr.push(my_file{
                    name: _name.to_string(),
                    path: _path.to_string(),
                });
                let my_file_ser = to_string(&arr[i]);
                if my_file_ser.is_ok(){
                    println!("{}",my_file_ser.ok().unwrap());
                }
                else {
                    println!("error serializing json");
                }
            }
            None => {
                println!("no match for \"/\"");
            }
        }
    }
    return arr;
}
