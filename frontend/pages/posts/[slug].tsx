import { useRouter } from 'next/router'
import { handleBlogError, useBlogPost } from 'hooks/useBlog'
import ReactMarkdown from 'react-markdown'

import Layout from 'components/layout'
import LoadingPage from 'components/loading-page'

export function Post() {
  const router = useRouter()
  const { slug } = router.query
  const { data, error } = useBlogPost(slug as string)

  handleBlogError(router, error)
  if (!data) {
    return <LoadingPage />
  }
  return (
    <Layout title={data.post.title} editSlug={slug as string}>
      <ReactMarkdown>{data.post.content}</ReactMarkdown>
    </Layout>
  )
}

export default Post
