<script>
    import { theme } from "../stores.js";
    export let type = "blue";
</script>

<button
    class={$$props.class + " " + type}
    class:dark={$theme === "dark"}
    class:light={$theme === "light"}
>
    <slot />
</button>

<style>
    button {
        border-radius: 25px;
        height: 50px;
        outline: none;
        border-style: solid;
        border-width: 0;
        display: flex;
        align-items: center;
        justify-content: center;
        transition: all 0.2s;
        line-height: 100%;
    }
    button.green.light {
        background-color: rgba(217, 240, 228, 1);
        color: rgba(31, 166, 97, 1);
    }
    button.green.dark {
        background-color: rgba(36, 57, 54, 1);
        color: rgba(99, 226, 183, 1);
    }

    button.green.light:hover, button.green.light:focus {
        filter: brightness(0.95);
    }
    button.green.dark:hover, button.green.dark:focus {
        filter: brightness(0.8);
    }

    button:active {
        transform: scale(0.95);
    }
</style>
