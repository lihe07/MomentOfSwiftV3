<script>
    // Stores & Apis
    import {createEventDispatcher} from "svelte";
    import {theme} from "../stores";
</script>

<div style="position: fixed;right: 20px;display: flex;align-items: center; z-index: 1; height: calc(100vh - 100px)">
    <div class="box" class:dark={$theme === "dark"} class:light={$theme === "light"}>
        PANEL
    </div>

</div>


<style>
    .box {
        border-radius: 20px;
        padding: 15px;
        width: 70px;
        height: max-content;
    }

    .box.dark {
        background-color: #212130;
        box-shadow: 0px 0px 4px rgba(0, 0, 0, .5);
    }

    .box.light {
        background-color: white;
        box-shadow: rgba(0, 0, 0, 0.08) 0px 4px 6px 0px, rgba(0, 0, 0, 0.05) 0px 0px 0px 1px;
    }
</style>
