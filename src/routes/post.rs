use rocket::serde::json::Json;
use rocket::State;
//use serde::Deserialize;
use crate::models::{NewPost, Post, PaginatedPosts, PaginationMeta};
//use std::borrow::Cow;
//use rocket::query::Query;


use crate::db::DbPool;
//use crate::models::{Post, NewPost};
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

/* 
#[derive(FromForm, Deserialize)]
pub struct PostQuery {
    page: Option<i64>,
    limit: Option<i64>,
    search: Option<String>,
}
*/    


#[get("/list_posts?<page>&<limit>&<search>")]
pub fn list_posts(
    pool: &State<DbPool>,
    page: Option<i64>,
    limit: Option<i64>,
    search: Option<String>,
) -> Json<PaginatedPosts> {
    use crate::schema::posts::dsl::*;
 
    let mut conn = pool.get().expect("DB pool error");
 
    let page = page.unwrap_or(1);
    let limit = limit.unwrap_or(10);
    let offset = (page - 1) * limit;
 
    let mut count_query = posts.into_boxed();
    let mut select_query = posts.into_boxed();
 
    // Apply search filter if `search` exists
    if let Some(term) = search {
        let like_pattern = format!("%{}%", term);
count_query = count_query.filter(title.like(like_pattern.clone()));
select_query = select_query.filter(title.like(like_pattern));
    }
 
    let total_docs: i64 = count_query
        .count()
        .get_result(&mut conn)
        .expect("Failed to count posts");
 
    let post_list = select_query
        .offset(offset)
        .limit(limit)
        .load::<Post>(&mut conn)
        .expect("Failed to load posts");
 
    let total_pages = (total_docs as f64 / limit as f64).ceil() as i64;
    let from = offset + 1;
    let to = offset + post_list.len() as i64;
 
    Json(PaginatedPosts {
        records: post_list,
        meta: PaginationMeta {
            current_page: page,
            per_page: limit,
            from,
            to,
            total_pages,
            total_docs,
        },
    })
}