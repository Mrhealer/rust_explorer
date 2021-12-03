#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::response::Redirect;
use rocket_contrib::templates::Template;
use log::info;



#[derive(serde::Serialize)]
struct Message {
    user: &'static str,
    body: &'static str
}

#[derive(serde::Serialize)]
struct BoardContext {
    current_user: Option<String>,
    messages: Vec<Message>,
    parent: &'static str
}

#[derive(serde::Serialize)]
struct AboutContext {
    parent: &'static str
}

#[get("/")]
fn index() -> Redirect {
    Redirect::to(uri!(board:"longkaka"))
}

#[get("/user/<username>")]
fn board(username: String) -> Template {
    Template::render("templates/index", &BoardContext {
        current_user: Some(username),
        messages: vec![Message{user: "userA", body: "This is the first test message."},
                        Message{user: "userB", body: "This is the second test message."}],
        parent: "layout"
    })
}

#[get("/about")]
fn about() -> Template {
    Template::render("about", &AboutContext {
        parent: "layout"
    })
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}