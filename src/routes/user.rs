use rocket::serde::json::Json;
use rocket::State;

use crate::db::DbPool;
use crate::models::{NewUser, User};
use crate::schema::users::dsl::*;
use diesel::prelude::*;

#[post("/create_user", data = "<user_data>")]
pub fn create_user(
    pool: &State<DbPool>,
    user_data: Json<NewUser>,
) -> Json<User> {
    let mut conn = pool.get().expect("DB pool error");

    diesel::insert_into(users)
        .values(&*user_data)
        .execute(&mut conn)
        .expect("Failed to insert user");
    
    /* 
    let created_user = users
        .order(id.desc())
        .first::<User>(&mut conn)
        .expect("Failed to fetch user");
    */

        let created_user = users
        .select((id, username, first_name, last_name))
        .order(id.desc())
        .first::<User>(&mut conn)
        .expect("Failed to fetch user");

    Json(created_user)
}
