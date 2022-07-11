<script>
    // Stores & APIs
    import {theme} from "../stores";
    import {fade} from "svelte/transition";
    import CardCreateNew from "../reusable/CardCreateNew.svelte";
</script>
<div in:fade class:dark={$theme === "dark"} class:light={$theme === "light"}>
    <div class="list">
        <CardCreateNew/>
    </div>
</div>

<style>
    .list {
        margin: 20px;
        display: grid;
        grid-template-columns: repeat(3, 1fr); /* specify 3 rows with same width */
        grid-auto-rows: 300px; /* don't know how many rows there are, but know each row is 300px height */
        gap: 20px;

    }
</style>
