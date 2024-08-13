// ./components/providers.js
'use client'

import { Provider } from 'jotai'

export const JotaiProviders = ({ children }:{children:any}) => {
  return (
    <Provider>
      {children}
    </Provider>
  )
}