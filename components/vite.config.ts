import { resolve } from 'path'
import { defineConfig, Plugin, ViteDevServer } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';
import path from "path";
import app from "./dummy-server";

const mode = process.env.NODE_ENV || 'development';
const prod = mode === 'production';

const customServerPlugin: Plugin = {
  name: 'custom-server',
  async configureServer(server: ViteDevServer) {
    // Get the Express app instance (ESM style)
    // const { default: app } = await importESM('./custom-server');

    // Create a custom middleware handler function
    const expressMiddleware = (req: any, res: any, next: any) => {
      app(req, res, next);
    };

    // Add Express middleware to Vite server
    server.middlewares.use(expressMiddleware);
  },
};

export default defineConfig({
  plugins: [
    svelte({
      /* plugin options */
      compilerOptions: {
        customElement: true,
        dev: !prod,
      }
    }),
    customServerPlugin,
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
        login: "src/login.ts",
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
