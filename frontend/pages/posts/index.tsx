import { GetStaticPropsContext } from 'next'
import Link from 'next/link'

import { useRouter } from 'next/router'
import {
  handleBlogError,
  BlogPostList,
  blogPostList,
  useBlogPostList,
} from 'hooks/useBlog'

import Layout from 'components/layout'
import LoadingPage from 'components/loading-page'

function ListPage({ posts }: { posts: BlogPostList['list'] }) {
  return (
    <Layout title="List">
      <ul>
        {posts.map((post) => {
          const { slug, title } = post
          return (
            <li key={slug}>
              <Link href={`/posts/${slug}`}>{title}</Link>
            </li>
          )
        })}
      </ul>
    </Layout>
  )
}

export function List({ posts }) {
  const router = useRouter()
  const { data, error } = useBlogPostList()

  handleBlogError(router, error)
  if (!data) {
    return <LoadingPage />
  }
  return <ListPage posts={posts} />
}

export async function getStaticProps(_cxt: GetStaticPropsContext) {
  const posts = blogPostList()
  return {
    props: { posts },
  }
}

export default List
