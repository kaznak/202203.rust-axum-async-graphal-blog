import { request, gql } from 'graphql-request'
import useSWR from 'swr'

import { API_ENDPOINT } from 'configs/app'

export function useBlogPostList() {
  const query = gql`
    query List {
      list {
        slug
        title
      }
    }
  `

  type PostList = {
    list: {
      slug: string
      title: string
    }[]
  }

  return useSWR<PostList>(query, (query) => request(API_ENDPOINT, query))
}

export function useBlogPost(slug: string) {
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

  type PostData = {
    post: {
      title: string
      content: string
    }
  }

  return useSWR<PostData>(query, (query) =>
    request(API_ENDPOINT, query, variables)
  )
}
