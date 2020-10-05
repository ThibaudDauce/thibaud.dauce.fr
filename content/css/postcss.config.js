module.exports = {
  plugins: [
    require('postcss-import'),
    require('tailwindcss')('./content/css/tailwind.config.js'),
    require('autoprefixer'),
  ]
}
