mod board;
use rocket::{fs::{FileServer, relative}, serde::json::Json};
use board::Board;

#[macro_use] extern crate rocket;

#[post("/solve", format = "json", data = "<req_data>")]
fn solve(req_data: Json<Board>) {
    println!("{:?}", req_data);
}

#[rocket::launch]
fn rocket() -> _ {
    rocket::build().mount("/", FileServer::from(relative!("frontend/build/"))).mount("/api", routes![solve])
}
