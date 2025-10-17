import { defineConfig } from 'vite'
import path from 'path'

export default defineConfig({
  build: {
    lib: {
      entry: path.resolve(__dirname, 'src/init.ts'),
      name: 'StasisInitializer',
      fileName: () => 'init.js',
      formats: ['iife']
    },
    outDir: 'dist',
    emptyOutDir: false,
  }
})