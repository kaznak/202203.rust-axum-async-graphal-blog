import { request, gql } from 'graphql-request'
import useSWR from 'swr'

import { API_ENDPOINT } from 'configs/app'

export type Post = {
  slug: string
  title: string
  content: string
}

/// List
export type BlogPostList = {
  list: Array<Pick<Post, 'slug' | 'title'>>
}

export function useBlogPostList() {
  const query = gql`
    query List {
      list {
        slug
        title
      }
    }
  `

  return useSWR<BlogPostList>(query, (query) => request(API_ENDPOINT, query))
}

/// Post
export type BlogPost = {
  post: Pick<Post, 'title' | 'content'>
}

export function useBlogPost(slug?: string) {
  const query = gql`
    query Post($slug: String!) {
      post(slug: $slug) {
        title
        content
      }
    }
  `

  const variables = {
    slug,
  }

  return useSWR<BlogPost>(
    slug ? { query, variables } : null,
    ({ query, variables }) => request(API_ENDPOINT, query, variables)
  )
}

/// Create
export type BlogCreate = {
  create: Post
}

export async function blogCreate(post: Post) {
  const query = gql`
    mutation BlogCreate($post: PostInput!) {
      create(post: $post) {
        slug
        title
        content
      }
    }
  `
  const variables = {
    post,
  }

  return request<BlogCreate>(API_ENDPOINT, query, variables)
}

/// Update
export type BlogUpdate = {
  update: Post
}

export async function blogUpdate(
  post: Pick<Post, 'slug'> & Partial<Omit<Post, 'slug'>>
) {
  const query = gql`
    mutation BlogUpdate($post: PostOpt!) {
      update(post: $post) {
        slug
        title
        content
      }
    }
  `
  const variables = {
    post,
  }

  return request<BlogUpdate>(API_ENDPOINT, query, variables)
}

/// Delete
export type BlogDelete = {
  delete: Post['slug']
}

export async function blogDelete(slug: string) {
  const query = gql`
    mutation BlogDelete($slug: ID!) {
      delete(slug: $slug)
    }
  `
  const variables = {
    slug,
  }

  return request<BlogDelete>(API_ENDPOINT, query, variables)
}
