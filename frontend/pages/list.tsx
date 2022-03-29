import Head from 'next/head'
import Link from 'next/link'
import Image from 'next/image'

import { request, gql } from 'graphql-request'
import useSWR from 'swr'

import { API_ENDPOINT } from 'configs/app'

import vercelLogoSvg from 'public/vercel.svg'

const query = gql`
  query List {
    list {
      slug
      title
    }
  }
`

type PostList = {
  list: {
    slug: string
    title: string
  }[]
}

export function List() {
  const { data, error } = useSWR<PostList>(query, (query) =>
    request(API_ENDPOINT, query)
  )

  return (
    <div className="flex flex-col items-center justify-center min-h-screen py-2">
      <Head>
        <title>List</title>
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <main className="w-full flex-1 px-20">
        <h1 className="text-6xl font-bold border-b-8">List</h1>

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
