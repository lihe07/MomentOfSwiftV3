<script>
    import {theme} from "../stores.js";

    export let type = "default";
    export let href;
</script>

<a class:dark={$theme === "dark"} class:light={$theme === "light"} {href} on:click class:pale={type==="pale"}>
    <slot/>
</a>

<style>
    a {
        text-decoration: none;
        font-size: 16px;
        font-weight: lighter;
        transition: all 0.2s;
        font-family: var(--font-sans-serif);
        letter-spacing: 2px;
        height: 22px;
        display: inline-block;
        line-height: 22px;
        text-align: center;
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

    a:focus {
        outline: none;
    }

    a:active {
        font-size: 14px;
    }

    .pale.dark {
        color: rgb(200, 200, 200);
    }

    .pale.light {
        color: rgb(100, 100, 100);
    }
</style>
