// src/models.rs

use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use crate::schema::posts;

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
