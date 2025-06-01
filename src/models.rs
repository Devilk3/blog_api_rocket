use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use crate::schema::posts;
use crate::schema::posts_tags;

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}


#[derive(Queryable, Serialize)]
pub struct Post {
    pub id: i32,
    pub created_by: i32,
    pub title: String,
    pub body: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = posts)]
pub struct NewPost {
    pub created_by: i32,
    pub title: String,
    pub body: String,
}

#[derive(Serialize)]
pub struct PaginatedPosts {
    pub records: Vec<PostWithTags>,
    pub meta: PaginationMeta,
}

#[derive(Serialize)]
pub struct PaginationMeta {
    pub current_page: i64,
    pub per_page: i64,
    pub from: i64,
    pub to: i64,
    pub total_pages: i64,
    pub total_docs: i64,
}

#[derive(Insertable)]
#[diesel(table_name = posts_tags)]
pub struct NewPostTag<'a> {
    pub fk_post_id: i32,
    pub tag: &'a str,
}

#[derive(Queryable, Serialize)]
pub struct PostWithTags {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub tags: Vec<String>,
    pub created_by: Option<UserInfo>, // Optional if created_by is null
}

#[derive(Deserialize)]
pub struct NewPostWithTags {
    pub created_by: i32,
    pub title: String,
    pub body: String,
    pub tags: Vec<String>,
}

#[derive(Serialize)]
pub struct UserInfo {
    pub user_id: i32,
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}
