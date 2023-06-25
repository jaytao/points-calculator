

#[macro_use] extern crate rocket;
use rocket::{
    serde::json::Json,
};
use serde::{Deserialize};

#[derive(Deserialize)]
struct UserCreate {
    id: String
}
#[post("/user", data = "<user>")]
fn index(user: Json<UserCreate>) -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}