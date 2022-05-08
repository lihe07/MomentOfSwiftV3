<script>
    // Stores & APIs
    import { theme } from "../stores";
</script>

<div
    class="main"
    class:dark={$theme === "dark"}
    class:light={$theme === "light"}
>
    <slot />
</div>

<style>
    .main {
        padding: 40px;
        border-radius: 20px;
        transition: all 0.2s;
    }
    .dark.main {
        background-color: #212130;
    }
    .light.main {
        background-color: white;
    }
</style>
