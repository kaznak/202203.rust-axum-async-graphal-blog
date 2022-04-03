import { request, gql } from 'graphql-request'
import useSWR from 'swr'
import { NextRouter } from 'next/router'

import { API_ENDPOINT } from 'configs/app'

export type Post = {
  slug: string
  title: string
  content: string
}

export function handleBlogError(router: NextRouter, error: any) {
  if (error) {
    if (
      error.response?.errors.some(({ message }) => 'Post Not Found' == message)
    ) {
      router.push('/404')
    } else {
      router.push('/500')
    }
  }
}

const fetcher = <T>({ query, variables }) =>
  request<T>(API_ENDPOINT, query, variables)

/// List
export type BlogPostList = {
  list: Array<Pick<Post, 'slug' | 'title'>>
}
const listRequest = gql`
  query List {
    list {
      slug
      title
    }
  }
`
export const useBlogList = () => {
  const key = { query: listRequest }
  return useSWR<BlogPostList>(key, fetcher)
}

/// Post
export type BlogPost = {
  post: Pick<Post, 'title' | 'content'>
}
const postRequest = gql`
  query Post($slug: String!) {
    post(slug: $slug) {
      title
      content
    }
  }
`
export const useBlogPost = (slug?: string) => {
  const key = slug ? { query: postRequest, variables: { slug } } : null
  return useSWR<BlogPost>(key, fetcher)
}

/// Create
export type BlogCreate = {
  create: Post
}
const createRequest = gql`
  mutation BlogCreate($post: PostInput!) {
    create(post: $post) {
      slug
      title
      content
    }
  }
`
export const useBlogCreate = (post: Post, mutate?) => {
  const { slug } = post

  const key = { query: createRequest, variables: { post } }
  const ret = fetcher<BlogCreate>(key)
  if (mutate) {
    mutate({ query: listRequest })
    mutate({ query: postRequest, variables: { slug } })
  }
  return ret
}
/// Update
export type BlogUpdate = {
  update: Post
}
const updateRequest = gql`
  mutation BlogUpdate($post: PostOpt!) {
    update(post: $post) {
      slug
      title
      content
    }
  }
`
export const useBlogUpdate = (
  post: Pick<Post, 'slug'> & Partial<Omit<Post, 'slug'>>,
  mutate?
) => {
  const { slug } = post

  const key = { query: updateRequest, variables: { post } }
  const ret = fetcher<BlogUpdate>(key)
  if (mutate) {
    mutate({ query: listRequest })
    mutate({ query: postRequest, variables: { slug } })
  }
  return ret
}

/// Delete
export type BlogDelete = {
  delete: Post['slug']
}
const deleteRequest = gql`
  mutation BlogDelete($slug: ID!) {
    delete(slug: $slug)
  }
`
export const useBlogDelete = (slug: string, mutate?) => {
  const key = { query: deleteRequest, variables: { slug } }
  const ret = fetcher<BlogDelete>(key)
  if (mutate) {
    mutate({ query: listRequest })
    mutate({ query: postRequest, variables: { slug } })
  }
  return ret
}
