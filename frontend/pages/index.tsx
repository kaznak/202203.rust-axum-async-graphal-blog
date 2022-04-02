import Layout from 'components/layout'
import Link from 'next/link'

export function Home() {
  return (
    <Layout title="rust-axum-async-graphql-blog">
      <Link href={`/posts`}>list</Link>
    </Layout>
  )
}

export default Home
