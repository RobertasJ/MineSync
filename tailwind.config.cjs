/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,svelte}",
  ],
  theme: {
    extend: {
      colors: {
        "bg-primary": "#2c3e50",
        "bg-secondary": "#1f2b38",
        "bg-tertiary": "#213243",
        "button-hover": "#e74c3c",
        "button-clicked": "#c0392b",
        "button-nothover": "#19232e",
        "title-bar": "#111820",
      },
    },
  },
  plugins: [],
  
}