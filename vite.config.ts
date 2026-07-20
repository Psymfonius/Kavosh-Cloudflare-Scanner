import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import tailwindcss from "@tailwindcss/vite";

const host = "127.0.0.1";

export default defineConfig(async () => ({
  plugins: [vue(), tailwindcss()],
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    host: host,
    hmr: { protocol: "ws", host: host, port: 1421 },
    watch: { ignored: ["**/src-tauri/**"] },
  },
}));