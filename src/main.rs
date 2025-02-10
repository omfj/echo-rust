use rocket::fs::{relative, FileServer};
use rocket_dyn_templates::{context, Template};

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> Template {
    let context = context! {};
    Template::render("index", context)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/static", FileServer::from(relative!("static")))
        .attach(Template::fairing())
}
