import test from 'node:test'
import assert from 'node:assert/strict'
import { formatRemaining } from './countdown.mjs'

test('formats remaining seconds as mm:ss', () => {
  assert.equal(formatRemaining(125), '02:05')
  assert.equal(formatRemaining(0), '00:00')
})
