use juniper::Context;
use rocket_sync_db_pools::{database, diesel};

#[database("rusty_shorts_db")]
pub struct RustyShortsDB(diesel::PgConnection);

impl Context for RustyShortsDB {}
