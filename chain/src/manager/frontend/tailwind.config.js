/** @type {import('tailwindcss').Config} */
export default {
  content: [
    './index.html',
    './src/**/*.svelte'
  ],
  theme: {
    extend: {},
  },
  plugins: [
    require('tailwindcss-animated'),
    require('daisyui'),
  ],
}

