use crate::backend::post::*;
use async_graphql::{Context, EmptySubscription, Object, Schema, SimpleObject, ID};
use futures::lock::Mutex;
use slab::Slab;

pub type GraphQLSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

#[derive(Clone, SimpleObject)]
pub struct Post {
    slug: ID,
    title: String,
    content: String,
}

impl Into<PostData> for Post {
    fn into(self) -> PostData {
        let Post {
            slug,
            title,
            content,
        } = self;
        PostData {
            title,
            slug: slug.to_string(),
            content,
        }
    }
}

impl From<PostData> for Post {
    fn from(postdata: PostData) -> Self {
        let PostData {
            title,
            slug,
            content,
        } = postdata;
        Post {
            slug: ID::from(slug),
            title,
            content,
        }
    }
}

pub type Storage = Mutex<Slab<Post>>;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn post<'ctx>(&self, ctx: &Context<'ctx>, slug: String) -> Result<Post, String> {
        let dummy_post = Post {
            slug: ID::from(slug),
            title: String::from("dummy"),
            content: String::from("dummy"),
        };
        Ok(dummy_post)
    }
    async fn list<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<Post>, String> {
        let dummy_post = Post {
            slug: ID::from("dummy"),
            title: String::from("dummy"),
            content: String::from("dummy"),
        };
        Ok(vec![dummy_post])
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        slug: String,
        title: String,
        content: String,
    ) -> Result<Post, String> {
        let dummy_post = Post {
            slug: ID::from(slug),
            title,
            content,
        };
        Ok(dummy_post)
    }
    async fn update<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        slug: String,
        title: String,
        content: String,
    ) -> Result<Post, String> {
        let dummy_post = Post {
            slug: ID::from(slug),
            title,
            content,
        };
        Ok(dummy_post)
    }
    async fn delete<'ctx>(&self, ctx: &Context<'ctx>, slug: String) -> Result<ID, String> {
        Ok(ID::from(slug))
    }
}
