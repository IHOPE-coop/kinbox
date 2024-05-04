import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';

const mode = process.env.NODE_ENV || 'development';
const prod = mode === 'production';

export default defineConfig({
    plugins: [
        svelte({
            /* plugin options */
            compilerOptions: {
                customElement: true,
                dev: !prod
            }
        })
    ],
    build: {
        lib: {
            entry: './src/main.js',
            name: 'svelte-app'
        },
        outDir: 'public/build'
    }
});