import { test, expect } from '@playwright/test'
import { BodyType, WireMock } from 'wiremock-captain'

test('should return a message for health check', async ({ request }) => {
  const res = await request.get('/ping')
  expect(res.status()).toEqual(200)
  expect((await res.body()).toString()).toEqual('pong')
})

test('should return Decent profile file from Visualizer', async ({
  request,
}) => {
  const mock = new WireMock('http://localhost:18080')
  await mock.clearAll()
  await mock.register(
    {
      method: 'GET',
      endpoint: '/api/shots/19a2039f-999e-4d55-8c0d-8ae472154e14/profile',
    },
    { status: 200, body: 'THIS IS AWESOME PROFILE' },
    { responseBodyType: 'body' as BodyType }
  )

  const res = await request.get(
    '/profiles/19a2039f-999e-4d55-8c0d-8ae472154e14?from=visualizer'
  )
  expect(res.status()).toEqual(200)
  expect((await res.body()).toString()).toEqual('THIS IS AWESOME PROFILE')
})
