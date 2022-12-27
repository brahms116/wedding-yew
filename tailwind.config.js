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
        "fade-splash-button": {
          "0%": { opacity: "0" },
          "60%": { opacity: "0", transform: "translateY(50%)" },
          "100%": { opacity: "1", transform: "translateY(0%)" },
        },
        "fade-splash": {
          "0%": { opacity:"1" },
          "98%": { opacity:"0" },
          "99%": { transform:"translateY(0%)", opacity:"0" },
          "100%": { transform:"translateY(100%)", opacity:"0" }
        }
      },
      animation: {
        fade: "fade 0.5s linear",
        "fade-splash-button": "fade-splash-button 1.5s ease-out",
        "fade-splash": "fade-splash 0.5s ease-in 1 normal forwards"
      },
    },
  },
  variants: {},
  plugins: [],
};
