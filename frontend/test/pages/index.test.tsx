import React from 'react'
import { render } from 'test/testUtils/testingLibraryWrapper'
import { Home } from 'pages/index'

describe('Home page', () => {
  it('matches snapshot', () => {
    const { asFragment } = render(<Home />, {})
    expect(asFragment()).toMatchSnapshot()
  })
})
