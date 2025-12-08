import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

// https://vite.dev/config/
export default defineConfig({
  plugins: [react()],
  resolve: {
    // @ が import の中に出てきたら /src に置き換える
    alias: {
      '@': '/src'
    },
  },
  server: {
    port: 5173,
  }
})
