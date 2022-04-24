<script>
    import Button from "./Button.svelte";
    export let reason;
    export let back_text;
    import { createEventDispatcher, onDestroy } from "svelte";
    const dispatch = createEventDispatcher();
    import { theme } from "../stores.js";
    import { onMount } from "svelte";

    function back(e) {
        if (e.key === "Backspace") {
            dispatch("click");
        }
    }

    onMount(() => {
        window.addEventListener("keydown", back);
    });
    onDestroy(() => {
        window.removeEventListener("keydown", back);
    });
</script>

<div
    class="main"
    class:dark={$theme === "dark"}
    class:light={$theme === "light"}
>
    <div class="error-icon">
        <svg
            xmlns="http://www.w3.org/2000/svg"
            xmlns:xlink="http://www.w3.org/1999/xlink"
            viewBox="0 0 16 16"
        >
            <g fill="none">
                <path
                    d="M8 2a6 6 0 1 1 0 12A6 6 0 0 1 8 2zm0 8a.75.75 0 1 0 0 1.5a.75.75 0 0 0 0-1.5zm0-5.5a.5.5 0 0 0-.492.41L7.5 5v3.5l.008.09a.5.5 0 0 0 .984 0L8.5 8.5V5l-.008-.09A.5.5 0 0 0 8 4.5z"
                    fill="currentColor"
                />
            </g>
        </svg>
    </div>
    <h2>错误</h2>
    <h3>{reason}</h3>
    <Button class="button" on:click>{back_text}</Button>
</div>

<style>
    .main {
        text-align: center;
    }
    .error-icon {
        margin-top: 20px;
    }
    .error-icon > svg {
        color: #ef5350;
        width: 100px;
    }
    h2 {
        margin: 0;
        font-family: var(--font-sans-serif);
        letter-spacing: 2px;
    }
    .dark h2 {
        color: rgba(255, 255, 255, 0.8);
    }
    .light h2 {
        color: rgba(0, 0, 0, 0.8);
    }
    h3 {
        margin: 0;
        font-family: var(--font-sans-serif);
        letter-spacing: 1px;
        font-weight: lighter;
    }
    .dark h3 {
        color: rgba(255, 255, 255, 0.8);
    }
    .light h3 {
        color: rgba(0, 0, 0, 0.8);
    }
    .main :global(.button) {
        width: 100%;
        margin-top: 20px;
        margin-bottom: 20px;
    }
</style>
