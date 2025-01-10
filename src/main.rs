#[macro_use]
extern crate rocket;

mod api;
mod models;
mod schema;

use rocket::fs::{FileServer, relative};
use rocket_sync_db_pools::{database, diesel};
use crate::api::shorten_url::shorten;
use crate::api::delete_slug::delete_slug;
use crate::api::redirect_to_original::redirect_to_original;
use models::url;


#[database("rusty_shorts_db")]
pub struct RustyShortsDB(diesel::PgConnection);

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(RustyShortsDB::fairing())
        .mount("/", FileServer::from(relative!("static")))
        .mount("/", routes![redirect_to_original])
        .mount("/api", routes![shorten])
        .mount("/api", routes![delete_slug])
}
