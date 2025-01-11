use juniper::{graphql_object, EmptySubscription, FieldResult, RootNode};
use crate::{utils::utils, RustyShortsDB};
use crate::url::{Url, NewUrl};
use diesel::prelude::*;
use crate::schema::urls::dsl::*;

pub struct QueryRoot;

#[graphql_object(context = RustyShortsDB)]
impl QueryRoot {
    async fn get_urls(context: &RustyShortsDB) -> FieldResult<Vec<Url>> {
        let result = context.run(|conn| {
            urls.load::<Url>(conn)
        }).await;

        match result {
            Ok(data) => Ok(data),
            Err(_) => Err("Could not fetch URLs".into()),
        }
    }
}

pub struct MutationRoot;

#[graphql_object(context = RustyShortsDB)]
impl MutationRoot {
    async fn shorten_url(context: &RustyShortsDB, original_url_str: String) -> FieldResult<Url> {
        let new_slug = utils::generate_slug(&original_url_str);
        let new_url = NewUrl { slug: new_slug.clone(), original_url: original_url_str };

        let result = context.run(move |conn| {
            diesel::insert_into(urls)
                .values(&new_url)
                .get_result::<Url>(conn)
        }).await;

        match result {
            Ok(url) => Ok(url),
            Err(_) => Err("Could not shorten URL".into()),
        }
    }

    async fn delete_slug(context: &RustyShortsDB, slug_to_delete: String) -> FieldResult<String> {
        let affected_rows = context.run(move |conn| {
            diesel::delete(urls.filter(slug.eq(slug_to_delete)))
                .execute(conn)
        })
        .await;
    
        match affected_rows {
            Ok(0) => Err("Slug not found".into()),
            Ok(_) => Ok(format!("Slug deleted successfully")),
            Err(_) => Err("Failed to delete slug".into())
        }
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<RustyShortsDB>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot, EmptySubscription::<RustyShortsDB>::new())
}
