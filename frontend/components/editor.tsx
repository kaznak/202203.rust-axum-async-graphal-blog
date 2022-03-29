import { useState } from 'react'
import { useRouter } from 'next/router'
import Layout from 'components/layout'

import { blogCreate, blogDelete } from 'hooks/useBlog'

interface EditorProps {
  pageTitle: string
  slug: string
  title: string
  content: string
}

export function Editor({
  pageTitle,
  slug,
  title,
  content,
}: Partial<EditorProps>) {
  const router = useRouter()
  const [input, setInput] = useState<Partial<EditorProps>>({
    slug: slug ? slug : '',
    title: title ? title : '',
    content: content ? content : '',
  })

  const changeHandler = (event: { target: any }) => {
    const target = event.target
    const value = target.value
    const name = target.name

    setInput({
      ...input,
      [name]: value,
    })
  }

  return (
    <Layout title={pageTitle ? pageTitle : 'Editor'}>
      <button
        onClick={async (_) =>
          blogCreate(input).then(() => {
            window.alert('posted')
            router.push(`/posts`)
          })
        }
        className="bg-blue-500 hover:bg-blue-700 text-white font-bold m-2 py-2 px-4 rounded"
      >
        Create/Update
      </button>
      <button
        onClick={async (_) =>
          blogDelete(slug).then(() => {
            window.alert('deleted')
            router.push(`/posts`)
          })
        }
        className="bg-red-500 hover:bg-red-700 text-white font-bold m-2 py-2 px-4 rounded"
      >
        Delete
      </button>
      <h2>Slug</h2>
      <input
        name="slug"
        type="text"
        value={input.slug}
        onChange={changeHandler}
        className="border-2"
        readOnly={!!slug}
      ></input>
      <h2>Title</h2>
      <input
        name="title"
        type="text"
        value={input.title}
        onChange={changeHandler}
        className="border-2"
      ></input>
      <h2>Content</h2>
      <textarea
        name="content"
        value={input.content}
        onChange={changeHandler}
        className="border-2"
      ></textarea>
    </Layout>
  )
}
export default Editor
