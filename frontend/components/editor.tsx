import { useState } from 'react'
import { useRouter } from 'next/router'
import { useSWRConfig } from 'swr'

import { Post, useBlogCreate, useBlogDelete } from 'hooks/useBlog'

export function Editor({ slug = '', title = '', content = '' }: Partial<Post>) {
  const router = useRouter()
  const { mutate } = useSWRConfig()
  const [input, setInput] = useState<Post>({
    slug,
    title,
    content,
  })

  const changeHandler = ({ target }) => {
    const { value, name } = target
    setInput({
      ...input,
      [name]: value,
    })
  }

  return (
    <>
      <button
        onClick={async (_) =>
          useBlogCreate(input, mutate).then(() => {
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
          useBlogDelete(slug, mutate).then(() => {
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
    </>
  )
}
export default Editor
