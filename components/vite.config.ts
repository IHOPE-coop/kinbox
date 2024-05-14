import { resolve } from 'path'
import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';
import path from "path";

const mode = process.env.NODE_ENV || 'development';
const prod = mode === 'production';

export default defineConfig({
  plugins: [
    svelte({
      /* plugin options */
      compilerOptions: {
        customElement: true,
        dev: !prod,
      }
    })
  ],
  build: {
    lib: {
      entry: ['src/starter.ts'],
      name: 'components',
      formats: ['es', 'cjs']
    },
    rollupOptions: {
      input: {
        starter: "src/starter.ts",
        app: "src/app.ts"
      },
      output: {
        inlineDynamicImports: false,
      }
    },
    // rollupOptions: {
    //   input: {
    //     main: resolve(__dirname, 'index.html'),
    //     nested: resolve(__dirname, 'nested/index.html'),
    //   }
    // }
  },
  resolve: {
    alias: {
      $lib: path.resolve("./src/lib"),
    },
  },
});
