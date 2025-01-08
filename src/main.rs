#[macro_use]
extern crate rocket;

use rocket_sync_db_pools::{database, diesel};
use diesel::prelude::*;

#[database("rusty_shorts_db")]
pub struct RustyShortsDB(diesel::PgConnection);

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(RustyShortsDB::fairing()) // Attach the database fairing
        .mount("/", routes![index]) // Mount routes
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
