import './app.css'
import Starter from './Starter.svelte'

const element = document.getElementById('app')!
const message = element.attributes.getNamedItem('message')!

const app = new Starter({
    target: element,
    props: {
        message: message.value
    }
})

export default app
