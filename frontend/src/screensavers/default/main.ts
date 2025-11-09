import './style.css'

const logoContainer = document.getElementById('logo-container') as HTMLElement
const logoShape = document.getElementById('logo-shape') as unknown as SVGElement

if (!logoContainer || !logoShape) {
  throw new Error('Element #logo-container or #logo-shape not found!')
}

let logoWidth = logoContainer.offsetWidth
let logoHeight = logoContainer.offsetHeight

let x = 0
let y = 0
let vx_pps: number
let vy_pps: number
let lastTimestamp = 0
const colors = [
  '#FF0000',
  '#00FF00',
  '#0000FF',
  '#FFFF00',
  '#00FFFF',
  '#FF00FF',
  '#FFFFFF',
  '#FFA500',
]
let currentColorIndex = 0

function changeColor() {
  let newIndex = currentColorIndex
  while (newIndex === currentColorIndex) {
    newIndex = Math.floor(Math.random() * colors.length)
  }
  currentColorIndex = newIndex
  logoShape.style.fill = colors[currentColorIndex]
}

function updateDimensions() {
  logoWidth = logoContainer.offsetWidth
  logoHeight = logoContainer.offsetHeight

  if (x + logoWidth > window.innerWidth) x = window.innerWidth - logoWidth
  if (y + logoHeight > window.innerHeight) y = window.innerHeight - logoHeight
}

function setSpeed() {
  const baseSpeed = 100
  const variableSpeed = window.innerWidth * 0.05
  const finalSpeed = baseSpeed + variableSpeed

  const currentDirX = vx_pps ? Math.sign(vx_pps) : 1
  const currentDirY = vy_pps ? Math.sign(vy_pps) : 1

  vx_pps = finalSpeed * currentDirX
  vy_pps = finalSpeed * currentDirY
}

function animate(timestamp: number) {
  if (lastTimestamp === 0) {
    lastTimestamp = timestamp
    requestAnimationFrame(animate)
    return
  }
  const deltaTime = (timestamp - lastTimestamp) / 1000
  lastTimestamp = timestamp

  const screenWidth = window.innerWidth
  const screenHeight = window.innerHeight

  x += vx_pps * deltaTime
  y += vy_pps * deltaTime

  let bounced = false

  if (x + logoWidth >= screenWidth || x <= 0) {
    vx_pps = -vx_pps
    if (x + logoWidth >= screenWidth) x = screenWidth - logoWidth
    if (x <= 0) x = 0
    bounced = true
  }

  if (y + logoHeight >= screenHeight || y <= 0) {
    vy_pps = -vy_pps
    if (y + logoHeight >= screenHeight) y = screenHeight - logoHeight
    if (y <= 0) y = 0
    bounced = true
  }

  if (bounced) {
    changeColor()
  }

  logoContainer.style.left = `${x}px`
  logoContainer.style.top = `${y}px`

  requestAnimationFrame(animate)
}

function start() {
  updateDimensions()

  if (logoWidth === 0 || logoHeight === 0) {
    setTimeout(start, 50)
    return
  }

  x = Math.random() * (window.innerWidth - logoWidth)
  y = Math.random() * (window.innerHeight - logoHeight)

  setSpeed()
  changeColor()
  requestAnimationFrame(animate)
}

start()

window.addEventListener('resize', () => {
  updateDimensions()
  setSpeed()
})