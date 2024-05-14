import './app.css'
import App from './App.svelte'

const element = document.getElementById('app')!
const current = element.attributes.getNamedItem('current')!
const other = element.attributes.getNamedItem('other')!

const app = new App({
    target: document.getElementById('app')!,
    props: {
        current: current.value,
        other: other.value,
    }
})

export default app

