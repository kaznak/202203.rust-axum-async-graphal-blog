import { useRouter } from 'next/router'
import { handleBlogError, useBlogPost } from 'hooks/useBlog'

import Layout from 'components/layout'
import { Editor } from 'components/editor'
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
    <Layout title="Update">
      <Editor
        slug={slug as string}
        title={data?.post.title}
        content={data?.post.content}
      />
    </Layout>
  )
}

export default Post
