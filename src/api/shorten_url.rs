use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use crate::RustyShortsDB;


#[derive(Deserialize)]
pub struct UrlRequest {
    url: String,
}

#[derive(Serialize)]
pub struct ShortenedResponse {
    slug: String,
    shortened_url: String,
}


#[post("/shorten", format = "json", data = "<req>")]
pub async fn shorten(db: RustyShortsDB, req: Json<UrlRequest>) -> Result<Json<ShortenedResponse>, String> {
    use crate::url::{NewUrl, Url};
    use crate::schema::urls;
    use crate::api::generate_slug::generate_slug;

    // 1. Generate slug
    let new_slug = generate_slug(&req.url);

    // 2. Build `NewUrl` with owned Strings
    let new_url = NewUrl {
        slug: new_slug,
        original_url: req.url.clone(),
    };

    // 3. Move `new_url` into the closure
    let insert_result = db.run(move |conn| {
        diesel::insert_into(urls::table)
            .values(new_url)           // <-- Pass by value, no refs
            .returning(Url::as_select())
            .get_result(conn)
    })
    .await;

    match insert_result {
        Ok(url) => {
            let response = ShortenedResponse {
                slug: url.slug.clone(),
                shortened_url: format!("http://localhost:8000/{}", url.slug),
            };
            Ok(Json(response))
        },
        Err(e) => Err(format!("Database error: {}", e)),
    }
}
