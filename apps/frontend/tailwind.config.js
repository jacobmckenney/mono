/** @type {import('tailwindcss').Config} */
module.exports = {
    content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
    theme: {
        extend: {},
        colors: {
            transparent: "transparent",
            // tertiary: "#B80C09",
            white: "#ffffff",
            black: "#141414",
            "gray-hover": "E2E2EA",
            gray: {
                0: "#FFFFFF",
                1: "#FAFAFB",
                2: "#F1F1F5",
                3: "#E2E2EA",
                4: "#D5D5DC",
                5: "#B5B5BE",
                6: "#92929D",
                7: "#8B8B96",
                8: "#696974",
                9: "#63636E",
                10: "#4F4F5A",
                11: "#44444F",
                12: "#292932",
                13: "#1C1C24",
                14: "#13131A",
            },
        },
    },
    plugins: [],
};
