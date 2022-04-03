use std::collections::HashSet;

use crate::datastore::{file::FileDataStore, post::*};
use async_graphql::{Context, EmptySubscription, InputObject, Object, Schema, SimpleObject};

pub type GraphQLSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

#[derive(Clone, SimpleObject, InputObject)]
#[graphql(input_name = "PostInput")]
pub struct Post {
    #[graphql(validator(min_length = 4, max_length = 1024, regex = r"^[a-z0-9-_]+$"))]
    slug: String,
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
            slug,
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
            slug: slug.clone(),
            title: title.clone(),
            content: content.clone(),
        }
    }
}

#[derive(Clone, SimpleObject, InputObject)]
#[graphql(input_name = "PostOptInput")]
pub struct PostOpt {
    #[graphql(validator(min_length = 4, max_length = 1024, regex = r"^[a-z0-9-_]+$"))]
    slug: String,
    title: Option<String>,
    content: Option<String>,
}

impl PostOpt {
    fn to_postdata(
        &self,
        datastore: &dyn DataStore,
    ) -> Result<PostData, Box<dyn std::error::Error>> {
        let PostData {
            slug,
            mut title,
            mut content,
        } = datastore.read_post(&self.slug.to_string())?;
        if let Some(v) = &self.title {
            title = v.to_string();
        }
        if let Some(v) = &self.content {
            content = v.to_string();
        }
        Ok(PostData {
            slug,
            title,
            content,
        })
    }
}

impl Into<PostOpt> for PostData {
    fn into(self) -> PostOpt {
        let PostData {
            slug,
            title,
            content,
        } = self;
        PostOpt {
            slug,
            title: Some(title),
            content: Some(content),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum BlogError {
    #[error("Post Not Found")]
    NotFound,
    #[error("Internal Server Error")]
    InternalServerError,
}

pub type Storage = FileDataStore;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn post<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        #[graphql(validator(min_length = 4, max_length = 1024, regex = r"^[a-z0-9-_]+$"))]
        slug: String,
    ) -> Result<Post, BlogError> {
        let datastore = ctx.data::<Storage>().unwrap();
        match datastore.read_post(&slug.as_str()) {
            Ok(p) => Ok(Post::from(p)),
            Err(e) => {
                if let Some(e) = e.downcast_ref::<std::io::Error>() {
                    if std::io::ErrorKind::NotFound == e.kind() {
                        return Err(BlogError::NotFound);
                    }
                }
                Err(BlogError::InternalServerError)
            }
        }
    }
    async fn list<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<PostOpt>, BlogError> {
        let datastore = ctx.data::<Storage>().unwrap();
        let slug_list = match datastore.list_posts() {
            Ok(v) => v,
            Err(_) => return Err(BlogError::InternalServerError),
        };

        let mut fields = ctx
            .field()
            .selection_set()
            .map(|field| field.name())
            .collect::<HashSet<_>>();

        fields.remove("slug");
        if 0 < fields.capacity() {
            log::trace!("list: fetch contents");
            return Ok(slug_list
                .iter()
                .map(|slug| datastore.read_post(&slug).unwrap().into())
                .collect());
        } else {
            log::trace!("list: only list");
            return Ok(slug_list
                .iter()
                .map(|slug| PostOpt {
                    slug: slug.clone(),
                    title: None,
                    content: None,
                })
                .collect());
        }
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create<'ctx>(&self, ctx: &Context<'ctx>, post: Post) -> Result<Post, BlogError> {
        let datastore = ctx.data::<Storage>().unwrap();
        let postdata = post.into();
        match datastore.create_post(&postdata) {
            Ok(p) => Ok(Post::from(p)),
            Err(_) => Err(BlogError::InternalServerError),
        }
    }
    async fn update<'ctx>(&self, ctx: &Context<'ctx>, postopt: PostOpt) -> Result<Post, BlogError> {
        let datastore = ctx.data::<Storage>().unwrap();
        let res_read = postopt.to_postdata(datastore);
        if let Err(_) = res_read {
            return Err(BlogError::InternalServerError);
        }
        let res_create = datastore.create_post(&res_read.unwrap());
        match res_create {
            Ok(p) => Ok(Post::from(p)),
            Err(_) => Err(BlogError::InternalServerError),
        }
    }
    async fn delete<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        #[graphql(validator(min_length = 4, max_length = 1024, regex = r"^[a-z0-9-_]+$"))]
        slug: String,
    ) -> Result<String, BlogError> {
        let datastore = ctx.data::<Storage>().unwrap();
        let slug_str = String::from(slug.clone());
        match datastore.delete_post(&slug_str) {
            Ok(_) => Ok(slug),
            Err(_) => Err(BlogError::InternalServerError),
        }
    }
}
