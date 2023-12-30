use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct paperPlane{
    name: String,
    file_type: String,
    path: String,
}

fn hello(){
    println!("hello mom");
}

