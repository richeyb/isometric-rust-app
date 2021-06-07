module.exports = {
  purge: {
    enabled: true, 
    content: [
      "./src/**/*.rs",
      "./static/*.html"
    ],
  },
  darkMode: false, // or 'media' or 'class'
  theme: {
    extend: {},
  },
  variants: {
    extend: {},
  },
  plugins: [],
}
