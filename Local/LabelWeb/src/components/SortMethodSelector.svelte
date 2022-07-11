<script>
    // Stores & APIs
    import {theme} from "../stores";
    import {createEventDispatcher, onDestroy, onMount} from "svelte";
    import {fly} from "svelte/transition";

    const dispatch = createEventDispatcher();


    export let current = "最新创建";
    $:others = ["最新创建", "最近更新", "最多数量"].filter(ele => ele !== current)

    let show_dropdown = false;
    let cooling_down = false;
    let select_cooling_down = false;
    let button;

    function check_blur(e) {
        if (cooling_down) {
            return;
        }
        // 如果点击了下拉框，则不处理
        if (e.target.classList.contains("drop-down")) {
            return;
        }
        // // 如果点击了下拉框的子元素，则不处理
        // if (e.target.parentElement.classList.contains("drop-down")) {
        //     return;
        // }
        // 如果点到了hr，则不处理
        if (e.target.tagName === "hr") {
            return;
        }
        show_dropdown = false;
        button.blur();
    }

    function select(id) {
        current = others[id];
        dispatch("select", current);
        select_cooling_down = true;
        setTimeout(() => select_cooling_down = false, 1000)
    }


    onMount(() => {
        window.addEventListener("click", check_blur);
        return () => {
            window.removeEventListener("click", check_blur);
        }
    })


</script>

<div class="container" class:dark={$theme === "dark"} class:light={$theme === "light"} class:select_cooling_down>
    <button on:focus={() => {
        if (cooling_down || select_cooling_down) {
            return;
        }
        show_dropdown = true;
        cooling_down = true;
        setTimeout(() => {
            cooling_down = false;
        }, 500);
    }} bind:this={button} class:focus={show_dropdown}>
        {current}
        <span class="icon">
            <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 24 24"><path
                    d="M7 10l5 5l5-5z" fill="currentColor"></path></svg>
        </span>
    </button>
</div>

{#if show_dropdown}
    <div class="drop-down" class:dark={$theme === "dark"} class:light={$theme === "light"}
         transition:fly={{duration: 400, y: -10}}>

        {#each others as other, i (other)}
            {#if i !== 0}
                <hr>
            {/if}
            <p class="drop-down-item" on:click={() => select(i)}>{other}</p>

        {/each}
    </div>
{/if}


<style>
    *:not(.icon *, .icon) {
        transition: all .2s;
    }

    button {
        border-radius: 20px;
        height: 50px;
        width: 120px;
        border: none;
        outline: none;
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

    .light:not(.select_cooling_down) button:hover, .light button.focus {
        color: black;
    }

    .dark:not(.select_cooling_down) button:hover, .dark button.focus {
        color: white;
    }

    button.focus {
        border: rgba(66, 165, 245, .6) 3px solid;
    }

    .icon svg {
        width: 20px;
        height: 20px;
        margin-top: 3px;
        margin-right: -5px;
        transition: transform .3s ease-in-out, color 0s;
    }


    .focus .icon svg {
        transform: rotate(180deg);
    }

    /*drop down*/
    .drop-down {
        width: 100px;
        position: absolute;
        margin-top: 10px;
        border-radius: 20px;
        padding: 5px 10px;
        z-index: 1;

    }

    .light.drop-down {
        background-color: #fff;
        box-shadow: rgba(0, 0, 0, 0.08) 0px 4px 6px 0px, rgba(0, 0, 0, 0.05) 0px 0px 0px 1px;
    }

    .dark.drop-down {
        background-color: #2E2E40;
        box-shadow: rgba(0, 0, 0, 0.3) 0px 4px 6px 0px, rgba(0, 0, 0, 0.2) 0px 0px 0px 1px;
    }

    .drop-down-item {
        font-family: var(--font-sans-serif);
        font-size: 16px;
        text-align: center;
        margin: 15px 0;
        letter-spacing: 1px;
        cursor: pointer;
        user-select: none;
    }

    .light .drop-down-item {
        color: rgba(0, 0, 0, 0.7);
    }

    .dark .drop-down-item {
        color: rgba(255, 255, 255, 0.8);
    }

    .light .drop-down-item:hover {
        color: black;
    }

    .dark .drop-down-item:hover {
        color: white;
    }

    .drop-down-item:active {
        scale: 0.95;
    }

    hr {
        margin: 0;
        border: none;
    }

    .light hr {
        border-top: 2px solid rgba(0, 0, 0, 0.1);
    }

    .dark hr {
        border-top: 2px solid rgba(255, 255, 255, 0.1);
    }

    .select_cooling_down button {
        opacity: 0.6;
        cursor: not-allowed;
    }


</style>
