#[macro_use] extern crate rocket;

mod hbs;

use rocket_dyn_templates::Template; // use in namespace

// http://127.0.0.1:8000/hello/world
#[get("/world")]
fn world() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/hello", routes![world])
        .mount("/hbs", routes![hbs::index, hbs::hello])
        .attach(Template::fairing())
}