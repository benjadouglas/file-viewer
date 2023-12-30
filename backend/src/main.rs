#[allow(unused)]
use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;
use std::path::Path;
use std::env;
use tower_http::cors::CorsLayer;
// use std::{ fs, io, };

#[tokio::main]
async fn main(){
    let app = Router::new()
        .route("/", get(index))
        //allow frontend to fetch data
        .layer(CorsLayer::very_permissive());
    let adress = SocketAddr::from(([127,0,0,1], 8080));
    axum::Server::bind(&adress)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index() -> String{
    let s = get_ex_dir("/benja/Code/C++");
    let mut ret = String::new();
    for i in s.iter(){
        ret.push_str(i);
    }
    ret
}


fn get_ex_dir(s:&str) -> Vec<String>{
    let home_dir = env::var("HOME").expect("HOME environment variable not set");
    // let home_dir = "/Users/benjamindouglas";
    let dir_path = format!("{}{}", home_dir,s);
    let dir = Path::new(&dir_path);

    let mut arr:Vec<String> = Vec::new(); 
    for entry in dir.read_dir().expect("Couldn't read dir") {
        match entry {
            Ok(entry) => {
                let path = entry.path();
                arr.push(path.to_str().unwrap().to_string());
            },
            Err(e) => println!("Error reading directory: {}", e),
        }
    }
    return arr
}
