import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import { viteSingleFile } from 'vite-plugin-singlefile'

export default defineConfig({
  plugins: [react(), viteSingleFile()],
  root: '__ROOT_DIR__',
  build: {
    outDir: '__OUT_DIR__',
    emptyOutDir: true,
    rollupOptions: {
      input: '__ENTRY_FILE__',
    },
  },
})