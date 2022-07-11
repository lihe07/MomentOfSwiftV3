<script>
    // Reusable
    import Card from "./Card.svelte";
    // Stores
    import {theme} from "../stores";

    import {createEventDispatcher} from "svelte";

    const dispatch = createEventDispatcher();

</script>

<Card clickable="true">
    <div class="container" class:dark={$theme === "dark"}
         class:light={$theme === "light"}>
        <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 12 12">
            <g fill="none">
                <path d="M6.5 1.75a.75.75 0 0 0-1.5 0V5H1.75a.75.75 0 0 0 0 1.5H5v3.25a.75.75 0 0 0 1.5 0V6.5h3.25a.75.75 0 0 0 0-1.5H6.5V1.75z"
                      fill="currentColor" style="--darkreader-inline-fill: currentColor;"
                      data-darkreader-inline-fill=""></path>
            </g>
        </svg>
        <p>创建新计数任务</p>
    </div>
</Card>

<style>
    .container {
        width: 100%;
        height: 100%;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
    }

    svg {
        width: 100px;
        height: 100px;
        transition: color .2s;
    }

    p {
        font-size: 20px;
        letter-spacing: 1px;
        transition: color .2s;
    }

    .dark {
        color: white;
        opacity: 90%;
    }
    .light {
        color: black;
        opacity: 95%;
    }



</style>
