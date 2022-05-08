<script>
    // Stores & APIs
    import {theme} from "../stores";
    import {createEventDispatcher} from "svelte";

    const dispatch = createEventDispatcher();

    let show_dropdown = false;

</script>

<div class="container" class:dark={$theme === "dark"} class:light={$theme === "light"}>
    <button on:focus={() => {show_dropdown = true;}} on:blur={() => {show_dropdown=false}}>
        最新创建
        <span class="icon">
            <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 24 24"><path
                    d="M7 10l5 5l5-5z" fill="currentColor"></path></svg>
        </span>
    </button>
</div>

{#if show_dropdown}
    <div class="drop-down" class:dark={$theme === "dark"} class:light={$theme === "light"}>
        Dropdown
    </div>
{/if}


<style>
    button {
        border-radius: 20px;
        height: 50px;
        width: 120px;
        border: none;
        outline: none;
        transition: all .2s;
        display: flex;
        align-items: center;
        justify-content: space-between;
        font-size: 15px;
        letter-spacing: 2px;
        cursor: pointer;
        padding: 15px;
        white-space: nowrap;
    }

    button:active {
        scale: 0.95;
    }

    .light button {
        background-color: #fff;
        border: 3px solid #fff;
        box-shadow: rgba(0, 0, 0, 0.08) 0px 4px 6px 0px, rgba(0, 0, 0, 0.05) 0px 0px 0px 1px;
        color: rgba(0, 0, 0, 0.7);
    }

    .dark button {
        background-color: #2E2E40;
        border: 3px solid #2E2E40;
        box-shadow: rgba(0, 0, 0, 0.3) 0px 4px 6px 0px, rgba(0, 0, 0, 0.2) 0px 0px 0px 1px;
        color: rgba(255, 255, 255, 0.7);
    }

    button:hover, button:focus {
        box-shadow: 0 0 0 rgba(0, 0, 0, 0);
    }

    .light button:hover, .light button:focus {
        color: black;
    }

    .dark button:hover, .dark button:focus {
        color: white;
    }

    button:focus {
        border: rgba(66, 165, 245, .6) 3px solid;
    }

    .icon svg {
        width: 20px;
        height: 20px;
        margin-top: 3px;
        margin-right: -5px;
    }
    .drop-down {
        position: absolute;
        margin-top: 10px;
        border-radius: 20px;
    }
    .light.drop-down {
        background-color: #fff;

    }
</style>
