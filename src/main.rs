#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use rocket::response::Redirect;
use rocket::request::Form;
use rocket_contrib::templates::Template;
use std::env;

use xz::schema::links::dsl::*;
use xz::models::Link;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}


#[get("/")]
fn list() -> String {
    let connection = establish_connection();
    let results = links.load::<Link>(&connection)
        .expect("Error loading links");
    for l in results {
        println!("{} {}", l.src, l.dst);
    }

    format!("Hello world")
} 

#[get("/<name>")]
fn hello(name: String) -> Redirect {
    let mut res: Redirect = Redirect::to(format!("/{}/new", name));

    let connection = establish_connection();
    let results = links.filter(src.eq(name))
        .load::<Link>(&connection)
        .expect("Error loading links");
    for l in results {
        res = Redirect::to(format!("{}", l.dst));
    }

    res
}

#[derive(FromForm)]
struct Formlink {
    dest: String,
}

#[post("/<name>/new", data = "<formlink>")]
fn new(name: String, formlink: Form<Formlink>) -> Redirect {
    use xz::create_link;
    let connection = establish_connection();

    create_link(&connection, &name, &formlink.dest);

    Redirect::to(format!("/{}", name))
}

#[derive(Serialize)]
struct TemplateContext {
    name: String,
}

#[get("/<name>/new")]
fn new_template(name: String) -> Template {
    let context = TemplateContext { name };
    Template::render("new", &context)
}

fn main() {
    rocket::ignite().attach(Template::fairing()).mount("/", routes![list, hello, new, new_template]).launch();
}
