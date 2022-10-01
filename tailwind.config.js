module.exports = {
  content: [
    "./src/**/*.rs",
    "./index.html",
    "./src/**/*.html",
    "./src/**/*.css",
  ],
  theme: {
    extend: {
      fontFamily: {
        sans: ["Cormorant Garamond"],
      },
      keyframes: {
        fade: {
          "0%": { opacity: "0" },
          "100%": { opacity: "1" },
        },
      },
      animation: {
        fade: "fade 1s linear",
      },
    },
  },
  variants: {},
  plugins: [],
};
