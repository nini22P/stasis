import { spawn } from 'child_process'
import { glob } from 'glob'
import path from 'path'
import fs from 'fs'
import { fileURLToPath } from 'url'

const __dirname = path.dirname(fileURLToPath(import.meta.url))
const VITE_EXECUTABLE = path.resolve(__dirname, 'node_modules', '.bin', 'vite')
const TEMP_CONFIG_DIR = path.resolve(__dirname, '.vite-temp-configs')

function runProcess(command) {
  const childProcess = spawn(command, { stdio: 'inherit', shell: true })
  return new Promise((resolve, reject) => {
    childProcess.on('close', (code) => {
      if (code === 0) resolve()
      else reject(new Error(`Process failed with exit code ${code}`))
    })
    childProcess.on('error', (err) => reject(err))
  })
}

function setupTempDir() {
  if (fs.existsSync(TEMP_CONFIG_DIR)) {
    fs.rmSync(TEMP_CONFIG_DIR, { recursive: true, force: true })
  }
  fs.mkdirSync(TEMP_CONFIG_DIR)
}

async function buildInitScript() {
  console.log('\nBuilding injector script (init.js)...')
  const command = `"${VITE_EXECUTABLE}.cmd" build --config vite.config.init.ts`
  await runProcess(command)
}

async function buildHtmlPages() {
  const entryPoints = await glob('src/**/index.html')
  const templateContent = fs.readFileSync('./vite.config.template.ts', 'utf-8')

  for (const entry of entryPoints) {
    console.log(entry)
    const isScreensaver = entry.includes('screensavers')
    const entryDir = path.dirname(entry)
    const entryName = path.basename(entryDir)

    let baseOutDir = 'dist'
    if (isScreensaver) {
      baseOutDir = path.join('dist', 'screensavers')
    }
    const outDir = path.resolve(__dirname, baseOutDir, entryName)

    console.log(`\nBuilding HTML page: ${entryName}...`)

    const newConfigContent = templateContent
      .replace(/__ROOT_DIR__/g, path.resolve(__dirname, entryDir).replace(/\\/g, '/'))
      .replace(/__OUT_DIR__/g, outDir.replace(/\\/g, '/'))
      .replace(/__ENTRY_FILE__/g, path.resolve(__dirname, entry).replace(/\\/g, '/'))

    const tempConfigPath = path.join(TEMP_CONFIG_DIR, `vite.config.${entryName}.ts`)
    fs.writeFileSync(tempConfigPath, newConfigContent)

    const command = `"${VITE_EXECUTABLE}.cmd" build --config "${tempConfigPath}"`
    await runProcess(command)
  }
}

async function main() {
  try {
    if (fs.existsSync('./dist')) {
      fs.rmSync('./dist', { recursive: true, force: true })
    }

    setupTempDir()

    await buildInitScript()
    await buildHtmlPages()

  } catch (error) {
    console.error(`\n❌ Build failed: ${error.message}`)
    process.exit(1)
  } finally {
    if (fs.existsSync(TEMP_CONFIG_DIR)) {
      fs.rmSync(TEMP_CONFIG_DIR, { recursive: true, force: true })
    }
  }
}

main()