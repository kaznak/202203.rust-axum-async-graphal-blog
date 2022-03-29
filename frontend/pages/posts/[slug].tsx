import { useRouter } from 'next/router'
import { useBlogPost } from 'hooks/useBlog'

import Layout from 'components/layout'

export function Post() {
  const router = useRouter()
  const { slug } = router.query
  const { data, error } = useBlogPost(slug as string)

  if (error) {
    return <div>failed to load</div>
  }
  if (!data) {
    return <div>loading...</div>
  }
  return (
    <Layout title={data.post.title} editSlug={slug as string}>
      {data.post.content}
    </Layout>
  )
}

export default Post
