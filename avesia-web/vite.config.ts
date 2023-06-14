import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react-swc'

// https://vitejs.dev/config/
export default defineConfig({
  server: {
    port: 8513,
  },
  build: {
    outDir: "../html",
  },
  plugins: [react()],
})
