<script>
    // Stores & APIs
    import {theme} from "../stores";
    import {fade} from "svelte/transition";
</script>

<div in:fade class:dark={$theme === "dark"} class:light={$theme === "light"}>
    My Tasks
</div>
