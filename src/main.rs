#[macro_use]
extern crate rocket;

mod api;
mod models;
mod schema;
mod graphql;
mod db;
mod utils;

use rocket::serde::json::Json;
use rocket::fs::{FileServer, relative};
use crate::api::redirect_to_original::redirect_to_original;
use crate::api::qr::get_qr;
use graphql::schema::{create_schema, Schema};
use db::RustyShortsDB;
use juniper::http::GraphQLRequest;
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
        .attach(RustyShortsDB::fairing())
        .mount("/", FileServer::from(relative!("static")))
        .mount("/", routes![get_qr])
        .mount("/", routes![redirect_to_original])
        .manage(schema)
        .mount("/", routes![
            graphql_handler,
            graphiql
        ])
}
