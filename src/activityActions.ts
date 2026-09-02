export const snoozeUntil = (now = new Date()) => new Date(now.getTime() + 5 * 60_000)
