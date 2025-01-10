use crate::schema::urls;
use diesel::prelude::*;
use juniper::GraphQLObject;


#[derive(Queryable, Selectable, GraphQLObject)]
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