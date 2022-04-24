<script>
    import { theme } from "../stores.js";

    let className;

    export { className as class };
</script>

<div
    class={"card " + className}
    class:dark={$theme === "dark"}
    class:light={$theme === "light"}
>
    <slot />
</div>

<style>
    .card {
        border-radius: 20px;
        padding: 40px;
        /* 深浅转换 .2s */
        transition: background-color 0.2s;
    }
    .card.dark {
        background-color: #212130;
    }
    .card.light {
        background-color: white;
    }
</style>
