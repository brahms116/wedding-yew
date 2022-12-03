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
        "bg":"#fafafa"
      },
      fontFamily: {
        sans: ["Cormorant Garamond"],
        serif: ["Dancing Script"],
      },
      keyframes: {
        fade: {
          "0%": { opacity: "0" },
          "100%": { opacity: "1" },
        },
      },
      animation: {
        fade: "fade 0.5s linear",
        "fade-slow": "fade 1s linear",
      },
    },
  },
  variants: {},
  plugins: [],
};
