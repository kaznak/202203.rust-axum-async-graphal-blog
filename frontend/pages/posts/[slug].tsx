import { useRouter } from 'next/router'
import { useBlogPost } from 'hooks/useBlog'

import Layout from 'components/layout'
import { Editor } from 'components/editor'

export function Post() {
  const router = useRouter()
  const { slug } = router.query
  const { data, error } = useBlogPost(slug as string)

  if (error) {
    // return <div>failed to load</div>
    console.log('editor')
    return (
      <Editor
        slug={slug as string}
        title={undefined}
        content={undefined}
      ></Editor>
    )
  }
  if (!data) {
    return <div>loading...</div>
  }
  return <Layout title={data.post.title}>{data.post.content}</Layout>
}

export default Post
