import { PlaywrightTestConfig } from '@playwright/test'

const config: PlaywrightTestConfig = {
  use: {
    baseURL: 'http://localhost:3000',
    extraHTTPHeaders: {
      Accept: 'plain/text',
    },
  },
}

export default config
