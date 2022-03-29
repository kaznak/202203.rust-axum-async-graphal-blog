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

  return useSWR<BlogPost>(slug ? query : null, (query) =>
    request(API_ENDPOINT, query, variables)
  )
}
