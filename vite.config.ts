import { defineConfig } from "vite";
import solid from "vite-plugin-solid";
import tailwindcss from "@tailwindcss/vite";
import path from "node:path";

import monacoEditorPlugin from "vite-plugin-monaco-editor";

const host = process.env.TAURI_DEV_HOST;

// https://vitejs.dev/config/
export default defineConfig(async () => ({
    // biome-ignore lint/suspicious/noExplicitAny: <explanation>
    plugins: [solid(), tailwindcss(), (monacoEditorPlugin as any).default({})],

    // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
    //
    // 1. prevent vite from obscuring rust errors
    clearScreen: false,
    // 2. tauri expects a fixed port, fail if that port is not available
    server: {
        port: 1420,
        strictPort: true,
        host: host || false,
        hmr: host
            ? {
                  protocol: "ws",
                  host,
                  port: 1421,
              }
            : undefined,
        watch: {
            // 3. tell vite to ignore watching `src-tauri`
            ignored: ["**/src-tauri/**"],
        },
    },
    resolve: {
        alias: {
            "@components": path.resolve(__dirname, "src/components"),
            "@schemas": path.resolve(__dirname, "src/schemas"),
            "@state": path.resolve(__dirname, "src/state"),
        },
    },
}));
