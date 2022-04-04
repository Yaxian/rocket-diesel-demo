use rocket::http::Status;

use crate::models::post::{NewPost, Post};
use crate::models::response::{Response, ResponseWithStatus};
use crate::{constants, PostsDbConn};

pub fn get_posts(conn: PostsDbConn) -> ResponseWithStatus {
    let result_posts = Post::get_posts(&conn);
    ResponseWithStatus {
        status_code: Status::Ok.code,
        response: Response {
            message: String::from(constants::SUCCESS),
            data: serde_json::to_value(result_posts).unwrap(),
        },
    }
}

pub fn create_post(new_post: NewPost, conn: PostsDbConn) -> ResponseWithStatus {
    let result = Post::create_post(new_post, &conn);
    ResponseWithStatus {
        status_code: Status::Ok.code,
        response: Response {
            message: String::from(constants::SUCCESS),
            data: serde_json::to_value(result).unwrap(),
        },
    }
}


pub fn delete_post(id: i32, conn: PostsDbConn) -> ResponseWithStatus {
  let result = Post::delete_post(id, &conn);
  ResponseWithStatus {
    status_code: Status::Ok.code,
    response: Response {
        message: String::from(constants::SUCCESS),
        data: serde_json::to_value(result).unwrap(),
    },
}
}