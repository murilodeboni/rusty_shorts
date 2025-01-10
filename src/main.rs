#[macro_use]
extern crate rocket;

mod api;
mod models;
mod schema;

use rocket_sync_db_pools::{database, diesel};
use diesel::prelude::*;
use crate::api::shorten_url::shorten;
use crate::api::delete_slug::delete_slug;
use crate::api::redirect_to_original::redirect_to_original;
use models::url;


#[database("rusty_shorts_db")]
pub struct RustyShortsDB(diesel::PgConnection);

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(RustyShortsDB::fairing()) // Attach the database fairing
        .mount("/", routes![index]) // Mount routes
        .mount("/", routes![redirect_to_original])
        .mount("/api", routes![shorten])
        .mount("/api", routes![delete_slug])
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
