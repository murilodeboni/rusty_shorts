#[macro_use]
extern crate rocket;

mod api;
mod models;
mod schema;
mod graphql;
mod db;

use rocket_sync_db_pools::{database, diesel};
use rocket::serde::json::Json;
use diesel::prelude::*;
use crate::api::shorten_url::shorten;
use crate::api::delete_slug::delete_slug;
use crate::api::redirect_to_original::redirect_to_original;
use models::url;
use graphql::schema::{create_schema, Schema};
use db::RustyShortsDB;
use juniper::http::{GraphQLRequest, GraphQLResponse};
use juniper::http::GraphQLResponse as JuniperResponse; // Explicit import

// use juniper_rocket::GraphQLResponse;
use rocket::{routes, State};

#[post("/graphql", data = "<request>")]
async fn graphql_handler(
    schema: &State<Schema>,
    db: RustyShortsDB,
    request: Json<GraphQLRequest>,
) -> Json<JuniperResponse> {
    let response = request.execute(schema, &db).await;
    Json(response)
}

#[get("/graphiql")]
fn graphiql() -> rocket::response::content::RawHtml<String> {
    juniper_rocket::graphiql_source("/graphql", None)
}



#[launch]
fn rocket() -> _ {
    let schema = create_schema();

    rocket::build()
        .attach(RustyShortsDB::fairing()) // Attach the database fairing
        .mount("/", routes![index]) // Mount routes
        .mount("/", routes![redirect_to_original])
        .mount("/api", routes![shorten])
        .mount("/api", routes![delete_slug])
        .manage(schema)
        .mount("/", routes![
            graphql_handler,
            graphiql
        ])
}

#[get("/")]
async fn index(db: RustyShortsDB) -> String {
    let result = db
        .run(|conn| {
            diesel::dsl::sql::<diesel::sql_types::BigInt>("SELECT COUNT(*) FROM urls")
                .get_result::<i64>(conn)
        })
        .await;

    match result {
        Ok(count) => format!("Number of URLs in DB: {}", count),
        Err(e) => format!("Error querying DB: {}", e),
    }
}
