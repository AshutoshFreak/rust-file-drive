use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct File {
    pub id: String,
    pub filename: String,
    pub content_type: String,
    pub size: i32,
    pub created_at: String,
    pub data: Vec<u8>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::files)]
pub struct NewFile<'a> {
    pub id: &'a str,
    pub filename: &'a str,
    pub content_type: &'a str,
    pub size: i32,
    pub data: &'a [u8],
}
