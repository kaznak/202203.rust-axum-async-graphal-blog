use crate::backend::{file::FileBackend, post::*};
use async_graphql::{Context, EmptySubscription, Object, Schema, SimpleObject, ID};

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

impl From<&PostData> for Post {
    fn from(postdata: &PostData) -> Self {
        let PostData {
            title,
            slug,
            content,
        } = postdata;
        Post {
            slug: ID::from(slug),
            title: title.clone(),
            content: content.clone(),
        }
    }
}

pub type Storage = FileBackend;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn post<'ctx>(&self, ctx: &Context<'ctx>, slug: String) -> Result<Post, String> {
        let backend = ctx.data::<Storage>().unwrap();
        match backend.read_post(&slug) {
            Ok(p) => Ok(Post::from(p)),
            Err(_) => Err("error!".to_string()),
        }
    }
    async fn list<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<Post>, String> {
        let backend = ctx.data::<Storage>().unwrap();
        match backend.list_posts() {
            Ok(l) => Ok(l.iter().map(|pd| Post::from(pd)).collect()),
            Err(_) => Err("error!".to_string()),
        }
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        slug: ID,
        title: String,
        content: String,
    ) -> Result<Post, String> {
        let backend = ctx.data::<Storage>().unwrap();
        let postdata = PostData {
            slug: String::from(slug),
            title,
            content,
        };
        match backend.create_post(&postdata) {
            Ok(p) => Ok(Post::from(p)),
            Err(_) => Err("error!".to_string()),
        }
    }
    async fn update<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        slug: ID,
        title: String,
        content: String,
    ) -> Result<Post, String> {
        let backend = ctx.data::<Storage>().unwrap();
        let postdata = PostData {
            slug: String::from(slug),
            title,
            content,
        };
        match backend.create_post(&postdata) {
            Ok(p) => Ok(Post::from(p)),
            Err(_) => Err("error!".to_string()),
        }
    }
    async fn delete<'ctx>(&self, ctx: &Context<'ctx>, slug: ID) -> Result<ID, String> {
        let backend = ctx.data::<Storage>().unwrap();
        let slug_str = String::from(slug.clone());
        match backend.delete_post(&slug_str) {
            Ok(_) => Ok(slug),
            Err(_) => Err("error!".to_string()),
        }
    }
}
