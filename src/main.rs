mod board;
use rocket::{fs::{FileServer, relative}, serde::json::Json};
use board::Board;

/*
#[macro_use] extern crate rocket;

#[post("/solve", format = "json", data = "<req_data>")]
fn solve(mut req_data: Json<Board>) {
    println!("{:?}", req_data.solve());
    println!("{:?}", req_data);
}

#[rocket::launch]
fn rocket() -> _ {
    rocket::build().mount("/", FileServer::from(relative!("frontend/build/"))).mount("/api", routes![solve])
}
*/

fn main() {
    let mut b: Board = Board::example();
    println!("{}", b.solve());
    println!("{:?}", b);
}
