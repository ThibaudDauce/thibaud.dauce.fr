const defaultTheme = require("tailwindcss/defaultTheme");

module.exports = {
  future: {
    removeDeprecatedGapUtilities: true,
    purgeLayersByDefault: true,
  },
  purge: [
    './build/**/*',
  ],
  theme: {
    extend: {
      fontFamily: {
        serif: ['Merriweather', ...defaultTheme.fontFamily.serif],
      },
      colors: {
        solarized: '#fdf6e3',
      },
    },
    typography: (theme) => ({
      default: {
        css: {
          pre: {
            color: 'inherit',
            backgroundColor: theme('colors.solarized'),
          },
          a: {
            color: theme('colors.indigo.600'),
            fontStyle: 'italic',
          },
        },
      },
    }),
  },
  variants: {},
  plugins: [
    require('@tailwindcss/typography'),
  ],
  corePlugins: {
    container: false,
  },
}
