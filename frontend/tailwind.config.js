/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
  theme: {
    extend: {
      spacing: {
        xxs: "4px",
        xs: "8px",
        sm: "16px",
        md: "24px",
        lg: "32px",
        xl: "48px",
        xxl: "64px"
      },
      colors: {
        primary: "#329af0",
        "primary-dark": "#257acf",
        secondary: "#ffffff",
        "secondary-dark": "#f8f9fa"
      }
    }
  },
  plugins: []
};
