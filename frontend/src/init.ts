declare global {
  interface Window {
    ready: () => void
    close: () => void
  }
}

let isLoaded = false

const handleClose = () => {
  if (isLoaded) {
    window.close()
  }
}

document.addEventListener('DOMContentLoaded', () => {
  window.ready()
  document.documentElement.style.cursor = 'none'
  setTimeout(() => {
    isLoaded = true
  }, 500)
})

window.addEventListener('mousemove', handleClose)
window.addEventListener('keydown', handleClose)
window.addEventListener('click', handleClose)
window.addEventListener('touchstart', handleClose)
window.addEventListener('touchmove', handleClose)