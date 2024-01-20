use rocket::{fs::{FileServer, relative}, serde::json::Json};
use serde::Deserialize;

#[macro_use] extern crate rocket;

#[derive(Deserialize)]
struct SudokuBoard {
    state: Vec<String>
}

#[post("/solve", format = "json", data = "<req_data>")]
fn solve(req_data: Json<SudokuBoard>) {
    println!("{:?}", req_data.state);
}

#[rocket::launch]
fn rocket() -> _ {
    rocket::build().mount("/", FileServer::from(relative!("frontend/build/"))).mount("/api", routes![solve])
}
