/** @type {import('tailwindcss').Config} */
export default {
    content: [
        "../packages/ui/src/web_app/**/*.{rs,html,css}",
        "../packages/ui/src/shared/**/*.{rs,html,css}",
        "../packages/web_app/src/**/*.{rs,html,css}"
    ],
    theme: {
        colors: {
            background: "#f5f5f5",
            foreground: "#1d293d",
            card: {
                DEFAULT: "#ffffff",
                foreground: "#1d293d",
            },
            popover: {
                DEFAULT: "#ffffff",
                foreground: "#1d293d",
            },
            primary: {
                DEFAULT: "#6468f0",
                foreground: "#ffffff",
            },
            secondary: {
                DEFAULT: "#e4e8ef",
                foreground: "#364050",
            },
            muted: {
                DEFAULT: "#f5f5f5",
                foreground: "#6b7280",
            },
            accent: {
                DEFAULT: "#e1e7fd",
                foreground: "#364050",
            },
            destructive: "#f14444",
            border: "#d0d4db",
            input: "#d0d4db",
            chart: {
                1: "#6468f0",
                2: "#4f46e5",
                3: "#443bc9",
                4: "#3730a5",
                5: "#312d84",
            },
            sidebar: "#ffffff",
        },
    }
}