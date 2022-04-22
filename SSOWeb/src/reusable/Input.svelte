<script>
    import { theme } from "../stores.js";

    let className;

    export { className as class };
    export let placeholder;
</script>

<input
    class:dark={$theme === "dark"}
    class:light={$theme === "light"}
    type="text"
    class={className}
    {placeholder}
/>

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
