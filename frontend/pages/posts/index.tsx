import Link from 'next/link'

import { useBlogPostList } from 'hooks/useBlog'

import Layout from 'components/layout'

export function List() {
  const { data, error } = useBlogPostList()

  if (error) {
    return <div>failed to load</div>
  }
  if (!data) {
    return <div>loading...</div>
  }
  return (
    <Layout title="List">
      {data.list.map((post) => (
        <li key={post.slug}>
          <Link href={`/posts/${post.slug}`}>{post.title}</Link>
        </li>
      ))}
    </Layout>
  )
}

export default List
