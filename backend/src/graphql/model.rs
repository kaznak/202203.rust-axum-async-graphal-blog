use crate::backend::{file::FileBackend, post::*};
use async_graphql::{Context, EmptySubscription, InputObject, Object, Schema, SimpleObject, ID};

pub type GraphQLSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

#[derive(Clone, SimpleObject, InputObject)]
#[graphql(input_name = "PostInput")]
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

#[derive(Clone, InputObject)]
pub struct PostOpt {
    slug: ID,
    title: Option<String>,
    content: Option<String>,
}

impl PostOpt {
    fn to_postdata(&self, backend: &dyn Backend) -> Result<PostData, Box<dyn std::error::Error>> {
        let PostData {
            slug,
            mut title,
            mut content,
        } = backend.read_post(&self.slug.to_string())?;
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
            Ok(ss) => {
                let posts = ss
                    .iter()
                    .map(|slug| Post::from(backend.read_post(&slug).unwrap()))
                    .collect();
                Ok(posts)
            }
            Err(_) => Err("error!".to_string()),
        }
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create<'ctx>(&self, ctx: &Context<'ctx>, post: Post) -> Result<Post, String> {
        let backend = ctx.data::<Storage>().unwrap();
        let postdata = post.into();
        match backend.create_post(&postdata) {
            Ok(p) => Ok(Post::from(p)),
            Err(_) => Err("error!".to_string()),
        }
    }
    async fn update<'ctx>(&self, ctx: &Context<'ctx>, postopt: PostOpt) -> Result<Post, String> {
        let backend = ctx.data::<Storage>().unwrap();
        let res_read = postopt.to_postdata(backend);
        if let Err(_) = res_read {
            return Err("error!".to_string());
        }
        let res_create = backend.create_post(&res_read.unwrap());
        match res_create {
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
