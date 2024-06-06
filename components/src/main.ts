import './app.css'
import App from './App.svelte'
import 'htmx.org'

const app = new App({
    target: document.getElementById('app')!,
})

export default app
