<script>
    import { theme } from "../stores.js";

</script>

<div
    class="main"
    class:dark={$theme === "dark"}
    class:light={$theme === "light"}
>
    <h2>成功</h2>
</div>

<style>
    .main {
        text-align: center;
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
