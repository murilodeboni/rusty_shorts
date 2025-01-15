use rocket::http::ContentType;
use crate::utils::qr_code::generate_qr_svg;

#[get("/qr?<data>")]
pub fn get_qr(data: &str, ) -> (ContentType, String) {
    (ContentType::new("image", "svg+xml"), generate_qr_svg(data))
}