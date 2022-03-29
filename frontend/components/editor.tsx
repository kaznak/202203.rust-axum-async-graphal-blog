import { ChangeEvent, useState } from 'react'
import Layout from 'components/layout'

interface EditorPorps {
  slug: string
  title: string
  content: string
}

export function Editor({ slug, title, content }: Partial<EditorPorps>) {
  const [input, setInput] = useState<Partial<EditorPorps>>({
    slug: slug ? slug : '',
    title: title ? title : '',
    content: content ? content : '',
  })

  const postHandler = () => {
    window.alert(`${input.slug} ${input.title} ${input.content}`)
  }
  const changeHandler = (event) => {
    const target = event.target
    const value = target.value
    const name = target.name

    setInput({
      ...input,
      [name]: value,
    })
  }

  return (
    <Layout title="Editor">
      <button
        onClick={postHandler}
        className="bg-blue-500 hover:bg-blue-700 text-white font-bold m-2 py-2 px-4 rounded"
      >
        Create/Update
      </button>
      <button
        onClick={postHandler}
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
