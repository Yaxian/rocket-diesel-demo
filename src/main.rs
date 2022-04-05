#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

#[macro_use]
extern crate diesel;
use diesel::pg::PgConnection;

#[macro_use]
extern crate diesel_migrations;

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
use rocket::fairing::AdHoc;
use rocket_contrib::database;

#[macro_use]
extern crate log;

mod api;
mod constants;
mod macros;
mod models;
mod schema;
mod services;

use api::post_controller::routes as post_routes;

embed_migrations!();
#[database("diesel_posts")]
pub struct PostsDbConn(PgConnection);

fn main() {
    rocket::ignite()
        .attach(PostsDbConn::fairing())
        .attach(AdHoc::on_attach("Database Migrations", |rocket| {
            let conn = PostsDbConn::get_one(&rocket).expect("Expect postgres database connection");
            match embedded_migrations::run(&*conn) {
                Ok(()) => Ok(rocket),
                Err(e) => {
                    error!("Failed to run database migrations: {:?}", e);
                    Err(rocket)
                }
            }
        }))
        .mount("/posts", post_routes())
        .launch();
}
