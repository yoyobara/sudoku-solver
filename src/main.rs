mod board;
use rocket::{fs::{FileServer, relative}, serde::json::Json, http::Status};
use board::Board;
use serde::Serialize;

#[macro_use] extern crate rocket;

#[derive(Serialize)]
struct SudokuResponse {
    ok: bool,
    board: Board
}

#[post("/solve", format = "json", data = "<req_data>")]
fn solve(mut req_data: Json<Board>) -> Result<Json<SudokuResponse>, Status> {

    if req_data.solve() {
        Ok(Json(SudokuResponse{ok: true, board: req_data.0}))
    } else {
        Err(Status::BadRequest)
    }
}

#[rocket::launch]
fn rocket() -> _ {
    rocket::build().mount("/", FileServer::from(relative!("frontend/build/"))).mount("/api", routes![solve])
}
