<script>
    import { onMount } from "svelte";

    import { theme } from "../stores.js";
    export let type = "blue";
    export let auto_focus = false;
    let dom;
    onMount(() => {
        if (auto_focus) {
            dom.focus();
        }
    });
</script>

<button
    class={$$props.class + " " + type}
    class:dark={$theme === "dark"}
    class:light={$theme === "light"}
    bind:this={dom}
    on:click
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
        line-height: 100%;
        letter-spacing: 2px;
        transition: all 0.2s;
    }
    button.green.light {
        background-color: rgba(217, 240, 228, 1);
        color: rgba(31, 166, 97, 1);
    }
    button.green.dark {
        background-color: rgba(36, 57, 54, 1);
        color: rgba(99, 226, 183, 1);
    }

    button.green.light:hover,
    button.green.light:focus {
        filter: brightness(0.95);
    }
    button.green.dark:hover,
    button.green.dark:focus {
        filter: brightness(0.8);
    }

    button.blue.light {
        background-color: #dbebfd;
        color: rgba(32, 128, 240, 1);
    }
    button.blue.dark {
        color: rgba(112, 192, 232, 1);
        background-color: rgba(55, 68, 80, 1);
    }
    button.blue.light:hover,
    button.blue.light:focus {
        filter: brightness(0.95);
    }
    button.blue.dark:hover,
    button.blue.dark:focus {
        filter: brightness(0.8);
    }

    button:active {
        transform: scale(0.95);
    }
</style>
