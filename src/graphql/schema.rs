use juniper::{graphql_object, EmptySubscription, FieldResult, RootNode};
use crate::RustyShortsDB;
use crate::url::{Url, NewUrl};
use diesel::prelude::*;

pub struct QueryRoot;

#[graphql_object(context = RustyShortsDB)]
impl QueryRoot {
    async fn get_urls(context: &RustyShortsDB) -> FieldResult<Vec<Url>> {
        use crate::schema::urls::dsl::*;

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
        use crate::schema::urls::dsl::*;

        let new_slug = nanoid::nanoid!(6);
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
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<RustyShortsDB>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot, EmptySubscription::<RustyShortsDB>::new())
}
