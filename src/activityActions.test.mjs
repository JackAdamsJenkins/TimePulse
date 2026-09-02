import test from 'node:test'
import assert from 'node:assert/strict'
import { snoozeUntil, shouldIgnore } from './activityActions.mjs'

test('snooze schedules the reminder five minutes later', () => {
  const now = new Date('2026-09-02T10:00:00Z')
  assert.equal(snoozeUntil(now).toISOString(), '2026-09-02T10:05:00.000Z')
})

test('ignore marks a reminder as ignored', () => assert.equal(shouldIgnore('Ignorer'), true))
