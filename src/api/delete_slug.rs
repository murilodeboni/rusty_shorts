use rocket::http::Status;
use rocket::serde::json::Json;
use diesel::prelude::*;
use crate::RustyShortsDB;
use crate::schema::urls::dsl::*;

#[derive(serde::Serialize)]
pub struct DeleteResponse {
    message: String,
}

#[delete("/delete/<slug_to_delete>")]
pub async fn delete_slug(db: RustyShortsDB, slug_to_delete: &str) -> Result<Json<DeleteResponse>, Status> {
    
    let slug_to_delete_string = slug_to_delete.to_string();

    let affected_rows = db.run(move |conn| {
        diesel::delete(urls.filter(slug.eq(slug_to_delete_string)))
            .execute(conn)
    })
    .await;

    match affected_rows {
        Ok(0) => Err(Status::NotFound),  // If no rows were deleted, return 404
        Ok(_) => Ok(Json(DeleteResponse {
            message: format!("Slug '{}' deleted successfully", slug_to_delete),
        })),
        Err(_) => Err(Status::InternalServerError),
    }
}
