import { request, gql } from 'graphql-request'
import useSWR from 'swr'

import { API_ENDPOINT } from 'configs/app'

type PostList = {
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
  const ret = useSWR<PostList>(query, (query) => request(API_ENDPOINT, query))
  return ret
}
