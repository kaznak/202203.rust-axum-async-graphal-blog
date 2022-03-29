import Head from 'next/head'
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
    slug: string;
    title: string;
  }[]
}

const getPosts = () => {
  const { data, error } = useSWR<PostList>(query, (query) => {
    let ret = request(API_ENDPOINT, query)
    console.log(ret)
    return ret
  }
  )

  console.log({ query, data, error })

  if (error) return <div>failed to load</div>
  if (!data) return <div>loading...</div>
  return data.list.map((post) => <li key={post.slug}>{post.title}</li>)
}

export function List() {
  return (
    <div className="flex flex-col items-center justify-center min-h-screen py-2">
      <Head>
        <title>List</title>
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <main className="w-full flex-1 px-20">
        <h1 className="text-6xl font-bold border-b-8">List</h1>

        {getPosts()}
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
