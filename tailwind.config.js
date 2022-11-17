module.exports = {
  content: [
    "./src/**/*.rs",
    "./index.html",
    "./src/**/*.html",
    "./src/**/*.css",
  ],
  theme: {
    extend: {
      colors: {
        "bg":"#f6f6f6"
      },
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
