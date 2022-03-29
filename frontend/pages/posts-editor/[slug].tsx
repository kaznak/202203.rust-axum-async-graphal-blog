import { useRouter } from 'next/router'
import { useBlogPost } from 'hooks/useBlog'

import { Editor } from 'components/editor'

export function Post() {
  const router = useRouter()
  const { slug } = router.query
  const { data } = useBlogPost(slug as string)

  return (
    <Editor
      pageTitle="Update"
      slug={slug as string}
      title={data ? data.post.title : undefined}
      content={data ? data.post.content : undefined}
    ></Editor>
  )
}

export default Post
