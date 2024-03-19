mod board;
use rocket::{fs::{FileServer, relative}, serde::json::Json, http::Status};
use board::Board;

#[macro_use] extern crate rocket;

#[post("/solve", format = "json", data = "<req_data>")]
fn solve(mut req_data: Json<Board>) -> Result<Json<Board>, Status> {

    if req_data.solve() {
        Ok(req_data)
    } else {
        Err(Status::BadRequest)
    }
}

#[rocket::launch]
fn rocket() -> _ {
    rocket::build().mount("/", FileServer::from(relative!("frontend/build/"))).mount("/api", routes![solve])
}
