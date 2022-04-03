import Link from 'next/link'

import { useRouter } from 'next/router'
import { handleBlogError, useBlogPostList } from 'hooks/useBlog'

import Layout from 'components/layout'
import LoadingPage from 'components/loading-page'

export function List() {
  const router = useRouter()
  const { data, error } = useBlogPostList()

  handleBlogError(router, error)
  if (!data) {
    return <LoadingPage />
  }

  const posts = data.list
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

export default List
