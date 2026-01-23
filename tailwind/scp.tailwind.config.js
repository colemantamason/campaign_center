/** @type {import('tailwindcss').Config} */
export default {
    content: ["../packages/ui/src/shared/**/*.{rs,html,css}", "../packages/websites/external/stop_communism/src/**/*.{rs,html,css}"],
    theme: {
        colors: {
            background: "#ffffff",
            foreground: "#0a0a0a",
            primary: {
                DEFAULT: "#171717",
                foreground: "#fafafa",
            },
            secondary: "#b32930",
            destructive: "#e7000b",
            border: "#e5e5e5",
            ring: "#a1a1a1",
        },
    },
}
