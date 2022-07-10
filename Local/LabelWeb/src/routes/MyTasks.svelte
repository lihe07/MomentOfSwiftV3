<script>
    // Stores & APIs
    import {theme} from "../stores";
    import {fade} from "svelte/transition";
    // Reusable
    import Pagination from "../reusable/Pagination.svelte";
    import Card from "../reusable/Card.svelte";
    import TaskCard from "../reusable/TaskCard.svelte";
</script>

<div in:fade class:dark={$theme === "dark"} class:light={$theme === "light"}>
    <div class="list">
        <Card>
            <div>
                <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 12 12">
                    <g fill="none">
                        <path d="M6.5 1.75a.75.75 0 0 0-1.5 0V5H1.75a.75.75 0 0 0 0 1.5H5v3.25a.75.75 0 0 0 1.5 0V6.5h3.25a.75.75 0 0 0 0-1.5H6.5V1.75z"
                              fill="currentColor" style="--darkreader-inline-fill: currentColor;"
                              data-darkreader-inline-fill=""></path>
                    </g>
                </svg>
                <h2>创建新计数任务</h2>
            </div>
        </Card>
    </div>
    <Pagination/>
</div>
