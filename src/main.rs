use rocket::fs::{FileServer, relative};

#[macro_use] extern crate rocket;

#[rocket::launch]
fn rocket() -> _ {
    rocket::build().mount("/", FileServer::from(relative!("frontend/build/")))
}
