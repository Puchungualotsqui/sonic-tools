/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./views/**/*.templ",
    "./components/**/*.templ",
    "./static/**/*.html",
  ],
  theme: {
    extend: {},
  },
  plugins: [require("daisyui")],
};
