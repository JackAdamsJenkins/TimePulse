export function playPop(enabled, createAudio = () => new Audio('/pop.mp3')) {
  if (!enabled) return
  const audio = createAudio()
  audio.currentTime = 0
  void audio.play().catch(() => {})
}
