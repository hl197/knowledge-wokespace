import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

export default defineConfig({
  plugins: [react()],
  clearScreen: false,
  resolve: { dedupe: ['react', 'react-dom'] },
  server: { host: '127.0.0.1', port: 18427, strictPort: true },
  envPrefix: ['VITE_', 'TAURI_'],
})
