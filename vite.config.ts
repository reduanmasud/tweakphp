import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import dotenv from 'dotenv'
import { resolve } from 'path'

dotenv.config()

// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST

// https://vitejs.dev/config/
export default defineConfig({
  root: 'src/',
  base: './',
  publicDir: resolve(__dirname, 'public'),
  plugins: [vue()],
  resolve: {
    alias: {
      '@': resolve(__dirname, 'src/'),
    },
  },
  server: {
    port: parseInt(process.env.VITE_SERVER_PORT || '4999'),
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: 'ws',
          host,
          port: parseInt(process.env.VITE_SERVER_PORT || '4999') + 1,
        }
      : undefined,
    watch: {
      // tell Vite to ignore watching `src-tauri`
      ignored: ['**/src-tauri/**'],
    },
  },
  // prevent Vite from obscuring rust errors
  clearScreen: false,
  build: {
    outDir: '../dist/',
    assetsDir: '.',
  },
})

