import { useRouter } from 'next/router'
import { useBlogPost } from 'hooks/useBlog'
import ReactMarkdown from 'react-markdown'

import Layout from 'components/layout'

export function Post() {
  const router = useRouter()
  const { slug } = router.query
  const { data, error } = useBlogPost(slug as string)

  if (error) {
    if (
      error.response.errors.some(({ message }) => 'Post Not Found' == message)
    ) {
      router.push('/404')
    } else {
      return <div>failed to load</div>
    }
  }
  if (!data) {
    return <div>loading...</div>
  }
  return (
    <Layout title={data.post.title} editSlug={slug as string}>
      <ReactMarkdown>{data.post.content}</ReactMarkdown>
    </Layout>
  )
}

export default Post
