<script>
    import { theme } from "../stores.js";
    export let href;
</script>

<a class:dark={$theme === "dark"} class:light={$theme === "light"} {href}>
    <slot />
</a>

<style>
    a {
        text-decoration: none;
        font-size: 16px;
        font-weight: lighter;
        transition: all 0.2s;
    }
    .dark {
        color: rgba(184, 231, 255, 1);
    }
    .light {
        color: rgba(72, 179, 232, 1);
    }
    .dark:hover,
    .dark:focus {
        filter: brightness(0.7);
    }
    .light:hover,
    .light:focus {
        filter: brightness(0.8);
    }
    a:active {
        transform: scale(0.95);
    }
</style>
