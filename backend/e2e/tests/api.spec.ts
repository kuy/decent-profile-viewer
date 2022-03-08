import { test, expect } from '@playwright/test'

test('should response health check', async ({ request }) => {
  const res = await request.get('/ping')
  expect(res.ok()).toBeTruthy()
  expect((await res.body()).toString()).toEqual('pong')
})
