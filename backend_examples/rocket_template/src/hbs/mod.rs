use rocket_dyn_templates::{Template};
use rocket::response::Redirect;

#[macro_use] mod context;

// redirection
#[get("/")]
pub fn index() -> Redirect {
    Redirect::to(uri!("/hbs", hello(name = "Default")))
}

// create a page from index.hbs template
// here name is a variable in index.hbs
#[get("/hello/<name>")]
pub fn hello(name: &str) -> Template {
    Template::render("index", context! {
        title: "Hello",
        name: Some(name),
        items: vec!["One", "Two", "Three"],
    })
}




