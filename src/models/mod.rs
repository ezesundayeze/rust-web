use super::schema::posts;
use diesel::{prelude::*};
use serde::{Serialize, Deserialize};

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = posts)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

