use rocket::serde::json::Json;
use rocket::State;
use rocket::http::Status;
use diesel::sql_types::{Integer, Text, Nullable};  
use crate::models::{NewPostWithTags, NewPostTag, PostWithTags};

use crate::db::DbPool;
use crate::schema::posts::id as post_id;
use crate::models::UserInfo;
//use crate::schema::posts_tags::dsl::{posts_tags};
use diesel::prelude::*;

#[post("/create_post", data = "<post_data>")]
pub fn create_post(
    pool: &State<DbPool>,
    post_data: Json<NewPostWithTags>,
) -> Json<PostWithTags> {
    use crate::schema::posts::dsl::*;
    use crate::schema::posts_tags::dsl::*;

    let mut conn = pool.get().expect("DB pool error");

    // Step 1: Insert into `posts` table
    diesel::insert_into(posts)
        .values((
            created_by.eq(&post_data.created_by),
            title.eq(&post_data.title),
            body.eq(&post_data.body),
        ))
        .execute(&mut conn)
        .expect("Failed to insert post");

    // Step 2: Get the newly inserted post's ID
    let new_post_id: i32 = posts
        .select(post_id)
        .order(post_id.desc())
        .first(&mut conn)
        .expect("Failed to fetch new post ID");

    // Step 3: Insert tags into `posts_tags`
    let new_tags: Vec<NewPostTag> = post_data
        .tags
        .iter()
        .map(|tag_value| NewPostTag {
            fk_post_id: new_post_id,
            tag: tag_value,
        })
        .collect();

    diesel::insert_into(posts_tags)
        .values(&new_tags)
        .execute(&mut conn)
        .expect("Failed to insert tags");

    // Step 4: Return response
    Json(PostWithTags {
        id: new_post_id,
        created_by: None,
        title: post_data.title.clone(),
        body: post_data.body.clone(),
        tags: post_data.tags.clone(),
    })
}


#[derive(QueryableByName)]
struct RawPost {
    #[sql_type = "Integer"]
    id: i32,

    #[sql_type = "Text"]
    title: String,

    #[sql_type = "Text"]
    body: String,

    #[sql_type = "Nullable<Text>"]
    tags: Option<String>,

    #[sql_type = "Nullable<Integer>"]
    user_id: Option<i32>,

    #[sql_type = "Nullable<Text>"]
    username: Option<String>,

    #[sql_type = "Nullable<Text>"]
    first_name: Option<String>,

    #[sql_type = "Nullable<Text>"]
    last_name: Option<String>,
}




#[rocket::get("/posts?<page>&<limit>&<search>")]
pub async fn list_posts(
    pool: &State<DbPool>, 
    page: Option<u32>, 
    limit: Option<u32>, 
    search: Option<String>
) -> Result<Json<Vec<PostWithTags>>, Status> {
    let page = page.unwrap_or(1);
    let limit = limit.unwrap_or(10);
    let offset = (page - 1) * limit;

    let like_pattern = search
        .map(|term| format!("%{}%", term))
        .unwrap_or_else(|| "%%".to_string());

    let mut conn = pool.get().expect("DB connection failed");

    let raw_posts: Vec<RawPost> = diesel::sql_query(
        "
        SELECT 
            p.id,
            p.title,
            p.body,
            GROUP_CONCAT(pt.tag) AS tags,
            u.id AS user_id,
            u.username,
            u.first_name,
            u.last_name
        FROM posts p
        LEFT JOIN posts_tags pt ON pt.fk_post_id = p.id
        LEFT JOIN users u ON p.created_by = u.id
        WHERE p.title LIKE ?
        GROUP BY p.id
        ORDER BY p.id DESC
        LIMIT ? OFFSET ?
        "
    )
    .bind::<Text, _>(like_pattern)
    .bind::<Integer, _>(limit as i32)
    .bind::<Integer, _>(offset as i32)
    .load::<RawPost>(&mut conn)
    .expect("Failed to load raw posts");

    let posts1: Vec<PostWithTags> = raw_posts
        .into_iter()
        .map(|raw| {
            let tags = raw.tags
                .unwrap_or_default()
                .split(',')
                .map(|s| s.trim().to_string())
                .collect::<Vec<_>>();

            let created_by = raw.user_id.map(|id1| UserInfo {
                user_id: id1,
                username: raw.username.unwrap_or_default(),
                first_name: raw.first_name,
                last_name: raw.last_name,
            });

            PostWithTags {
                id: raw.id,
                title: raw.title,
                body: raw.body,
                tags,
                created_by,
            }
        })
        .collect();

    Ok(Json(posts1))  // ✅ This line returns the final API response
}