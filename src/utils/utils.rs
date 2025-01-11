use slug::slugify;
use nanoid::nanoid;

pub fn generate_slug(url: &str) -> String {
    let base = slugify(url);
    let random_part = nanoid!(4);
    format!("{}-{}", base, random_part)
}