use axum::{extract::Query, response::Json, routing::get, Router, http::{StatusCode,Response}};
use serde::{Deserialize, Serialize};
use std::{env, net::SocketAddr, path::Path, process::Command};
use tower_http::cors::CorsLayer;
// use serde_json::to_string;

#[derive(Debug, Deserialize)]
struct req_handler {
    file: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct my_file {
    name: String,
    ftype: String,
}

#[derive(Serialize, Deserialize)]
pub struct my_dir {
    path: String,
    files: Vec<my_file>,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/open/terminal", get(open_terminal))
        //allow frontend to fetch data
        .layer(CorsLayer::very_permissive());
    let adress = SocketAddr::from(([127, 0, 0, 1], 8080));
    axum::Server::bind(&adress)
        .serve(app.into_make_service())
        .await
        .unwrap();
} 

async fn index(Query(params): Query<req_handler>) -> Json<my_dir> {
    // let arr = serial_killer("/benja/Code/web/file_viewer/frontend/");
    let temp = params.file.as_deref().unwrap_or("");
    println!("In fn index temp= {}", temp);
    let arr = serial_killer(&format!("{}", temp));
    Json(arr)
}

fn serial_killer(s: &str) -> my_dir {
    let _files = filesV2(s);
    let mut arr: Vec<my_file> = Vec::new();
    for file in _files.iter() {
        arr.push(my_file {
            ftype: file.0.to_string(),
            name: file.1.to_string(),
        });
    }
    let obj = my_dir {
        path: s.to_string(),
        files: arr,
    };
    return obj;
}


fn files(s: &str) -> Vec<String> {
    let home_dir = env::var("HOME").expect("HOME environment variable not set");
    let dir_path = format!("{}/{}", home_dir, s);
    let dir = Path::new(&dir_path);
    let mut arr: Vec<String> = Vec::new();
    for entry in dir.read_dir().expect("Couldn't read dir") {
        match entry {
            Ok(entry) => {
                let name = entry.file_name();
                let ftype = entry.file_type();
                arr.push(name.to_str().unwrap().to_string());
            }
            Err(e) => println!("Error reading directory: {}", e),
        }
    }
    return arr;
}

fn filesV2(s: &str) -> Vec<(String, String)> {
    let home_dir = env::var("HOME").expect("HOME environment variable not set");
    println!("in filesV2 home_dir={}", home_dir);
    let dir_path = format!("{}/{}", home_dir, s);
    println!("in filesV2 dir_path={}", dir_path);
    let dir = Path::new(&dir_path);
    let mut arr: Vec<(String, String)> = Vec::new();
    for entry in dir.read_dir().expect("Couldn't read dir") {
        match (entry) {
            Ok(entry) => {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_dir() {
                        arr.push((
                            String::from("Directory"),
                            entry.file_name().to_str().unwrap().to_string(),
                        ))
                    } else if file_type.is_file() {
                        arr.push((
                            String::from("File"),
                            entry.file_name().to_str().unwrap().to_string(),
                        ))
                    } else {
                        arr.push((
                            String::from("Non-UTF8"),
                            entry.file_name().to_str().unwrap().to_string(),
                        ))
                    }
                } else {
                    println!("type:Unknown| file:{}", entry.file_name().to_str().unwrap())
                }
            }
            Err(e) => println!("Error reading directory: {}", e),
        }
    }
    return arr;
}

async fn open_terminal(Query(params): Query<req_handler>) -> &'static str{
    let s = params.file.as_deref().unwrap_or("/");
    let script = format!(
        "tell application \"iTerm\"
            set newWindow to (create window with default profile)
            tell newWindow
                tell the current session
                    write text \"cd ~{} && vim .\"
                end tell
            end tell
        end tell",
        s
    );
    Command::new("osascript")
        .arg("-e")
        .arg(script)
        .spawn()
        .expect("Failed to open terminal");
    return "ok";
}
