let mix = require('laravel-mix')

require('laravel-mix-purgecss');
let tailwindcss = require('tailwindcss');

mix.setPublicPath('./build')
   .postCss('content/css/app.css', 'css', [
      tailwindcss('content/css/tailwind.js'),
   ])
   .purgeCss({
      folders: ['build'],
      extensions: ['html'],
      whitelistPatternsChildren: [/content/],
   })
