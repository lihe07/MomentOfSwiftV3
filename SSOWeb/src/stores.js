import { writable } from "svelte/store";

function get_theme() {
    let theme = matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light";
    if (["light", "dark"].includes(localStorage.getItem("theme"))) {
        theme = localStorage.getItem("theme");
    }
    return theme;
}

export const theme = writable(get_theme());

theme.subscribe(value => {
    localStorage.setItem("theme", value);
})