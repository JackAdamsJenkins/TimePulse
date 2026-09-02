import test from 'node:test'
import assert from 'node:assert/strict'
import { playPop } from './sound.mjs'

test('plays a short pop only when enabled', () => {
  let starts = 0
  const audio = { currentTime: 0, play: () => { starts += 1; return Promise.resolve() } }
  playPop(true, () => audio)
  playPop(false, () => audio)
  assert.equal(starts, 1)
})
