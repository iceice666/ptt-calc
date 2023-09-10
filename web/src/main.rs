mod b30;

#[macro_use]
extern crate rocket;

// use rocket::fs::{relative, FileServer};

#[get("/")]
fn root() -> &'static str {
    "Hello, world!"
}
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![root, crate::b30::main])
}
