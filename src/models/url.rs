use crate::schema::urls;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = urls)]
pub struct Url {
    pub slug: String,
    pub original_url: String,
}

#[derive(Queryable, Insertable, Selectable, QueryableByName)]
#[table_name = "urls"]
pub struct NewUrl {
    pub slug: String,
    pub original_url: String 
}