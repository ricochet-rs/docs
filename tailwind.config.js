/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    relative: true,
    files: ["*.html", "./src/**/*.rs", ".src/docs/*.html"],
  },
  theme: {
    extend: {},
  },
  plugins: [],
};
