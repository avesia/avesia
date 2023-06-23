import { defineConfig } from 'vite'
import wasm from 'vite-plugin-wasm'
import react from '@vitejs/plugin-react-swc'

// https://vitejs.dev/config/
export default defineConfig({
  server: {
    port: 8513,
  },
  build: {
    outDir: "../../html",
    target: ['esnext', 'edge89', 'firefox89', 'chrome89', 'safari15'],
  },
  plugins: [react(), wasm()],
})
