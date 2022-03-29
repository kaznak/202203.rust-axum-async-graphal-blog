import { request, gql } from 'graphql-request'
import useSWR from 'swr'

import { API_ENDPOINT } from 'configs/app'

export type PostList = {
  list: {
    slug: string
    title: string
  }[]
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

  return useSWR<PostList>(query, (query) => request(API_ENDPOINT, query))
}

export type PostData = {
  post: {
    title: string
    content: string
  }
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

  return useSWR<PostData>(slug ? query : null, (query) =>
    request(API_ENDPOINT, query, variables)
  )
}
