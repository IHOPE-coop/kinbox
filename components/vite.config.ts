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
      entry: 'src/main.ts',
      name: 'components',
      formats: ['es', 'cjs']
    },
    rollupOptions: {
      input: {
        main: "src/main.ts",
        secondary: "src/secondary.ts"
      },
      output: {
        inlineDynamicImports: false
      }
    },
  },
  resolve: {
    alias: {
      $lib: path.resolve("./src/lib"),
    },
  },
});
