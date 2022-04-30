<script>
    import {theme} from "../stores.js";
    import {createEventDispatcher, onMount} from "svelte";

    const dispatch = createEventDispatcher();
    let className;

    export {className as class};
    let dom;
    export let placeholder;
    export let value = "";
    export let auto_focus = false;
    export let type = "text";
    onMount(() => {
        if (auto_focus) {
            dom.focus();
        }
    });
</script>

{#if type === "text"}
<input
        class:dark={$theme === "dark"}
        class:light={$theme === "light"}
        type="text"
        class={className}
        {placeholder}
        bind:value
        bind:this={dom}
        on:keydown={e => {
        if (e.key === "Enter") {
            dispatch("enter")
        }
    }}
/>
{:else if type === "password"}
<input
        class:dark={$theme === "dark"}
        class:light={$theme === "light"}
        type="password"
        class={className}
        {placeholder}
        bind:value
        bind:this={dom}
        on:keydown={e => {
        if (e.key === "Enter") {
            dispatch("enter")
        }
    }}
/>
{/if}

<style>
    input {
        border-radius: 25px;
        height: 50px;
        outline: none;
        border-width: 1px;
        transition: all 0.2s;
        padding-left: 20px;
        padding-right: 20px;
        border-style: none;
        font-size: 16px;
        letter-spacing: 1px;
    }

    input.dark:focus {
        /* transform: scale(1.03); */
        background-color: hsl(240, 8%, 12%);
    }

    input.light:focus {
        background-color: hsl(0, 0%, 90%);
    }

    /*  */
    input.dark {
        background-color: rgba(23, 23, 30, 1);
        color: white;
    }

    input.light {
        background-color: rgba(242, 242, 242, 1);
        color: black;
    }
</style>
