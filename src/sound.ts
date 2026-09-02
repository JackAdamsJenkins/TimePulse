export function playPop(enabled: boolean) {
  if (!enabled) return
  const context = new AudioContext()
  const oscillator = context.createOscillator()
  const gain = context.createGain()
  oscillator.frequency.value = 620
  gain.gain.setValueAtTime(0.08, context.currentTime)
  gain.gain.exponentialRampToValueAtTime(0.001, context.currentTime + 0.09)
  oscillator.connect(gain).connect(context.destination)
  oscillator.start()
  oscillator.stop(context.currentTime + 0.09)
  oscillator.addEventListener('ended', () => void context.close(), { once: true })
}
