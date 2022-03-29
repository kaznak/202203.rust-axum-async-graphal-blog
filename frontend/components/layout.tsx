import Head from 'next/head'
import Link from 'next/link'
import Image from 'next/image'

import vercelLogoSvg from 'public/vercel.svg'

export function Layout({ title, children }) {
  return (
    <div className="flex flex-col items-center justify-center min-h-screen py-2">
      <Head>
        <title>{title}</title>
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <main className="w-full flex-1 px-20">
        <Link href={`/posts`}>goto list</Link>
        <h1 className="text-6xl font-bold border-b-8">{title}</h1>
        <section>{children}</section>
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

export default Layout
