import Head from 'next/head'
import Link from 'next/link'
import Image from 'next/image'

import { useBlogPostList } from 'hooks/useBlog'

import vercelLogoSvg from 'public/vercel.svg'

export function List() {
  const { data, error } = useBlogPostList()

  return (
    <div className="flex flex-col items-center justify-center min-h-screen py-2">
      <Head>
        <title>List</title>
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <main className="w-full flex-1 px-20">
        <h1 className="text-6xl font-bold border-b-4">List</h1>

        {error ? (
          <div>failed to load</div>
        ) : !data ? (
          <div>loading...</div>
        ) : (
          data.list.map((post) => (
            <li key={post.slug}>
              <Link href={`/posts/${post.slug}`}>{post.title}</Link>
            </li>
          ))
        )}
      </main>

      <footer className="flex items-center justify-center w-full h-24 border-t">
        <a
          className="flex items-center justify-center"
          href="https://vercel.com"
          target="_blank"
          rel="noopener noreferrer"
        >
          Powered by{' '}
          <Image
            src={vercelLogoSvg}
            alt="Vercel Logo"
            className="h-4 ml-2"
            width={70.75}
            height={16}
          />
        </a>
      </footer>
    </div>
  )
}

export default List
