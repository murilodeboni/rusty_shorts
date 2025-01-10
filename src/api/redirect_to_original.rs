use diesel::prelude::*;
use crate::RustyShortsDB;


#[get("/<slug>")]
pub async fn redirect_to_original(db: RustyShortsDB, slug: &str) -> Result<rocket::response::Redirect, rocket::http::Status> {
    use crate::schema::urls::dsl::{urls, slug as db_slug, original_url};

    let slug_string = slug.to_string();

    let result = db.run(move |conn| {
        urls
            .filter(db_slug.eq(slug_string))
            .select(original_url)
            .first::<String>(conn)
    })
    .await;

    match result {
        Ok(original) => Ok(rocket::response::Redirect::to(original)),
        Err(_) => Err(rocket::http::Status::InternalServerError),
    }
}
