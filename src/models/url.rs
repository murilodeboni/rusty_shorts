use super::schema::urls;
use chrono::{NaiveDateTime};

#[derive(Queryable)]
pub struct Url {
    pub id: i32,
    pub slug: String,
    pub original_url: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "urls"]
pub struct NewUrl<'a> {
    pub slug: &'a str,
    pub original_url: &'a str,
}