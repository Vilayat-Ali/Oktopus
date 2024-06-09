// global styles
import './styles/globals.css';

// App component
import App from './App.svelte';

const app = new App({
  target: document.getElementById('app')!,
})

export default app
