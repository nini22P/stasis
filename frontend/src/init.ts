declare global {
  interface Window {
    ready: () => void
    quit: () => void
  }
}

let isLoaded = false
const MOVE_THRESHOLD = 50

let startX: number | null = null
let startY: number | null = null

const handleClose = () => {
  if (isLoaded) {
    window.quit()
  }
}

const handleMove = (event: MouseEvent | TouchEvent) => {
  event.preventDefault()
  event.stopPropagation()
  event.stopImmediatePropagation()

  if (!isLoaded) {
    return
  }

  let currentX: number
  let currentY: number

  if ('touches' in event && event.touches.length > 0) {
    currentX = event.touches[0].clientX
    currentY = event.touches[0].clientY
  } else if ('clientX' in event) {
    currentX = event.clientX
    currentY = event.clientY
  } else {
    return
  }

  if (startX === null || startY === null) {
    startX = currentX
    startY = currentY
    return
  }

  const deltaX = currentX - startX
  const deltaY = currentY - startY
  const distance = Math.sqrt(deltaX * deltaX + deltaY * deltaY)

  if (distance >= MOVE_THRESHOLD) {
    document.documentElement.style.cursor = 'default'
    handleClose()
  }
}

document.addEventListener('DOMContentLoaded', () => {
  window.ready()
  document.documentElement.style.cursor = 'none'
  setTimeout(() => {
    isLoaded = true
  }, 500)
})

window.addEventListener('mousemove', handleMove)
window.addEventListener('keydown', handleClose)
window.addEventListener('click', handleClose)
window.addEventListener('touchstart', handleClose)
window.addEventListener('touchmove', handleMove)