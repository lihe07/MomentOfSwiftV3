<script>
    // Stores & APIs
    import {theme} from "../stores";
    import {fade} from "svelte/transition";
    // Reusable
    import Pagination from "../reusable/Pagination.svelte";
    import Card from "../reusable/Card.svelte";
    import TaskCard from "../reusable/TaskCard.svelte";
    import CardCreateNew from "../reusable/CardCreateNew.svelte";
</script>

<div in:fade class:dark={$theme === "dark"} class:light={$theme === "light"}>
    <div class="list">
        <CardCreateNew/>
        <TaskCard/>
        <TaskCard num="114514"/>
        <TaskCard num="1919810"/>
        <TaskCard num="Infinite"/>
        <TaskCard num="1145141919810"/>
    </div>
    <Pagination/>
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
