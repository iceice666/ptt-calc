mod score;

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
        .mount("/", routes![root, crate::score::b30,crate::score::list ])
}
