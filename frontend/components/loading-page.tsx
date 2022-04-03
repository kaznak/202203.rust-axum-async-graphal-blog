import Layout from 'components/layout'

export function LoadingPage({ title = 'loading' }) {
  return (
    <Layout title={title}>
      <div className="text-4xl text-center">loading...</div>
    </Layout>
  )
}
export default LoadingPage
