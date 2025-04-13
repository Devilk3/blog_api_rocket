use rocket::serde::json::Json;
use rocket::State;

use crate::db::DbPool;
use crate::models::{Post, NewPost};
use crate::schema::posts::dsl::*;
use diesel::prelude::*;

#[post("/create_post", data = "<post_data>")]
pub fn create_post(
    pool: &State<DbPool>,
    post_data: Json<NewPost>,
) -> Json<Post> {
    let mut conn = pool.get().expect("DB pool error");

    diesel::insert_into(posts)
        .values(&*post_data)
        .execute(&mut conn)
        .expect("Failed to insert post");

    let created_post = posts
        .select((id, created_by, title, body))
        .order(id.desc())
        .first::<Post>(&mut conn)
        .expect("Failed to fetch post");

    Json(created_post)
}
