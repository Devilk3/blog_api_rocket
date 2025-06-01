use rocket::serde::json::Json;
use rocket::State;
use diesel::dsl::sql;
use diesel::sql_types::{Integer, Text, Nullable};  
use crate::models::{NewPostWithTags, NewPostTag, PostWithTags};
use crate::db::DbPool;
use crate::schema::posts::id as post_id;
use diesel::prelude::*;
use rocket::http::Status;
use crate::models::UserInfo;
use crate::models::PaginatedPosts;
use crate::models::PaginationMeta;


#[post("/create_post", data = "<post_data>")]
pub fn create_post(
    pool: &State<DbPool>,
    post_data: Json<NewPostWithTags>,
) -> Json<PostWithTags> {
    use crate::schema::posts::dsl::*;
    use crate::schema::posts_tags::dsl::*;

    let mut conn = pool.get().expect("DB pool error");

    // Step 1: Inserting into 'posts' table
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




#[rocket::get("/posts?<page>&<limit>&<search>")]
pub async fn list_posts(
    pool: &State<DbPool>,
    page: Option<u32>,
    limit: Option<u32>,
    search: Option<String>,
) -> Result<Json<PaginatedPosts>, Status> {
    use crate::schema::{posts, posts_tags, users};

    let page = page.unwrap_or(1);
    let limit = limit.unwrap_or(10);
    let offset = (page - 1) * limit;

    let like_pattern = search
        .map(|term| format!("%{}%", term))
        .unwrap_or_else(|| "%%".to_string());

    let mut conn = pool.get().expect("DB connection failed");

    let results = posts::table
        .left_join(users::table.on(posts::created_by.eq(users::id)))
        .left_join(posts_tags::table.on(posts_tags::fk_post_id.eq(posts::id)))
        .filter(posts::title.like(&like_pattern))
        .group_by(posts::id)
        .order_by(posts::id.desc())
        .limit(limit.into())
        .offset(offset.into())
        .select((
            posts::id,
            posts::title,
            posts::body,
            sql::<Nullable<Text>>("GROUP_CONCAT(DISTINCT posts_tags.tag)"),
            sql::<Nullable<Integer>>("MAX(users.id)"),
            sql::<Nullable<Text>>("MAX(users.username)"),
            sql::<Nullable<Text>>("MAX(users.first_name)"),
            sql::<Nullable<Text>>("MAX(users.last_name)"),
        ))
        .load::<(
            i32,
            String,
            String,
            Option<String>,
            Option<i32>,
            Option<String>,
            Option<String>,
            Option<String>,
        )>(&mut conn)
        .unwrap();

    let from = offset + 1;
    let to = offset + results.len() as u32;

    let posts = results
        .into_iter()
        .map(|row| {
            let (id, title, body, tags, user_id, username, first_name, last_name) = row;

            let tags = tags
                .unwrap_or_default()
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();

            let created_by = user_id.map(|uid| UserInfo {
                user_id: uid,
                username: username.unwrap_or_default(),
                first_name,
                last_name,
            });

            PostWithTags {
                id,
                title,
                body,
                tags,
                created_by,
            }
        })
        .collect::<Vec<PostWithTags>>();

    // Create the meta struct
    let meta = PaginationMeta {
        current_page: page as i64,
        per_page: limit as i64,
        from: from as i64,
        to: to as i64,
        total_pages: 0, // Placeholder for now
        total_docs: 0,  // Placeholder for now
    };

    let response = PaginatedPosts {
        records: posts,
        meta,
    };

    Ok(Json(response))
}
