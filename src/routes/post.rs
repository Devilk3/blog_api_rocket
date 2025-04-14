use rocket::serde::json::Json;
use rocket::State;
use diesel::sql_types::Nullable;

use crate::models::{PaginatedPosts, PaginationMeta};
use crate::models::{NewPostWithTags, NewPostTag, PostWithTags};

use crate::db::DbPool;
use crate::schema::posts::id as post_id;
use crate::schema::posts::dsl::posts;
//use crate::schema::posts_tags::dsl::{posts_tags};
use crate::schema::posts::dsl::*;
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
        created_by: post_data.created_by,
        title: post_data.title.clone(),
        body: post_data.body.clone(),
        tags: post_data.tags.clone(),
    })
}



#[get("/list_posts?<page>&<limit>&<search>")]
pub fn list_posts(
    pool: &State<DbPool>,
    page: Option<i64>,
    limit: Option<i64>,
    search: Option<String>,
) -> Json<PaginatedPosts> {
  //  use diesel::dsl::sql;
    use diesel::sql_types::{Integer, Text};

    let mut conn = pool.get().expect("DB pool error");

    let page = page.unwrap_or(1);
    let limit = limit.unwrap_or(10);
    let offset = (page - 1) * limit;

    let like_pattern = search
        .as_ref()
        .map(|term| format!("%{}%", term))
        .unwrap_or("%%".to_string());

        #[derive(QueryableByName)]
        struct RawPost {
            #[sql_type = "Integer"]
            #[diesel(column_name = id)]
            post_id1: i32,
        
            #[sql_type = "Integer"]
            #[diesel(column_name = created_by)]
            post_created_by1: i32,
        
            #[sql_type = "Text"]
            #[diesel(column_name = title)]
            post_title: String,
        
            #[sql_type = "Text"]
            #[diesel(column_name = body)]
            post_body: String,
        
            #[sql_type = "Nullable<Text>"]
            #[diesel(column_name = tags)]
            tag_list: Option<String>,
        }

    let raw_posts: Vec<RawPost> = diesel::sql_query(
        r#"
        SELECT p.id, p.created_by, p.title, p.body,
               GROUP_CONCAT(pt.tag, ',') as tags
        FROM posts p
        LEFT JOIN posts_tags pt ON p.id = pt.fk_post_id
        WHERE p.title LIKE ?
        GROUP BY p.id
        ORDER BY p.id DESC
        LIMIT ? OFFSET ?
    "#,
    )
    .bind::<Text, _>(like_pattern)
    .bind::<Integer, _>(limit as i32)
    .bind::<Integer, _>(offset as i32)
    .load(&mut conn)
    .expect("Failed to load posts with tags");

    let post_list: Vec<PostWithTags> = raw_posts
        .into_iter()
        .map(|raw| PostWithTags {
            id: raw.post_id1,
            created_by: raw.post_created_by1,
            title: raw.post_title,
            body: raw.post_body,
            tags: raw
                .tag_list
                .unwrap_or_default()
                .split(',')
                .map(|s| s.to_string())
                .collect(),
        })
        .collect();

    let total_docs: i64 = posts
        .filter(title.like(search.unwrap_or("%%".to_string())))
        .count()
        .get_result(&mut conn)
        .expect("Failed to count posts");

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