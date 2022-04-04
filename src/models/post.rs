use diesel::prelude::*;
use diesel::PgConnection;
use serde::{Deserialize, Serialize};

use crate::schema::posts;
use crate::schema::posts::dsl::*;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub is_deleted: bool,
}

#[derive(Serialize, Deserialize)]
pub struct NewPost {
    pub title: String,
    pub body: String,
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPostColumn {
    pub title: String,
    pub body: String,
    pub is_deleted: bool,
}


impl Post {
    pub fn get_posts(conn: &PgConnection) -> Vec<Post> {
        let query_posts = posts
            .filter(is_deleted.eq(false))
            .limit(5)
            .load::<Post>(conn)
            .unwrap();
        query_posts
    }

    pub fn create_post(new_post: NewPost, conn: &PgConnection) -> bool {
        let new_post = NewPostColumn {
            title: new_post.title,
            body: new_post.body,
            is_deleted: false,
        };

        let result = diesel::insert_into(posts)
            .values(&new_post)
            .execute(conn)
            .is_ok();
        result
    }

    pub fn delete_post(post_id: i32, conn: &PgConnection) -> bool {
        let result = diesel::update(posts.find(post_id))
            .set(is_deleted.eq(true))
            .execute(conn)
            .is_ok();
        result
    }
}
