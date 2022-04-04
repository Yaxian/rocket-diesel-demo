use crate::models::post::NewPost;
use crate::models::response::Response;
use crate::services::post_service;
use crate::{PostsDbConn, status_custom};
use rocket::Route;
use rocket::{http::Status, response::status};
use rocket_contrib::json::Json;

pub fn routes() -> Vec<Route> {
    routes![get_posts, create_post, delete_post]
}


#[get("/", format = "json")]
fn get_posts(conn: PostsDbConn) -> status::Custom<Json<Response>> {
    let response = post_service::get_posts(conn);
    status_custom!(response)
}

#[post("/create", format = "json", data = "<new_post>")]
fn create_post(new_post: Json<NewPost>, conn: PostsDbConn) -> status::Custom<Json<Response>> {
    let response = post_service::create_post(new_post.0, conn);
    status_custom!(response)
}

#[delete("/delete/<id>")]
fn delete_post(id: i32, conn: PostsDbConn) -> status::Custom<Json<Response>> {
    let response = post_service::delete_post(id, conn);
    status_custom!(response)
}
