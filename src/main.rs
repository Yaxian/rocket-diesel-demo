#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

#[macro_use]
extern crate diesel;
use diesel::pg::PgConnection;

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
use rocket_contrib::database;

mod api;
mod constants;
mod models;
mod schema;
mod services;
mod macros;

use api::post_controller::routes as post_routes;

#[database("diesel_posts")]
pub struct PostsDbConn(PgConnection);

fn main() {
    rocket::ignite()
        .attach(PostsDbConn::fairing())
        .mount("/posts", post_routes())
        .launch();
}
