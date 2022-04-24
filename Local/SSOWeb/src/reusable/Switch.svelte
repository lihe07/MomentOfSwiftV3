<script>
    import { createEventDispatcher } from "svelte";
    import { theme } from "../stores.js";

    const dispatch = createEventDispatcher();
    export let checked = true;
    let dom;
</script>

<div
    class={"switch " + $$props.class}
    class:checked
    on:click={() => {
        checked = !checked;
        dispatch("change", checked);
    }}
    class:dark={$theme === "dark"}
    class:light={$theme === "light"}
> 
    <div
        class="circle"
        bind:this={dom}
        style:width={dom && dom.getBoundingClientRect().height + "px"}
    />
</div>

<style>
    .switch {
        cursor: pointer;
        text-indent: -9999px;
        width: 50px;
        height: 25px;
        background: grey;
        display: block;
        border-radius: 100px;
        position: relative;
        /* 深浅转换 .2s */
        transition: background-color 0.2s;
    }

    .dark {
        background: rgba(164, 191, 200, 0.15);
    }
    .light {
        background: rgba(0, 191, 255, 0.15);
    }

    .dark .circle {
        background: rgba(196, 196, 196, 1);
    }
    .light .circle {
        background: rgba(255, 250, 168, 1);
    }

    .circle {
        content: "";
        position: absolute;
        top: 5px;
        left: 5px;
        width: 10px;
        height: calc(100% - 10px);
        /* background: #fff; */
        border-radius: 90px;
        /* 深浅转换 .2s */
        transition: background-color 0.2s, all 0.3s;
    }



    .switch.checked .circle {
        left: calc(100% - 5px);
        transform: translateX(-100%);
    }

    .switch:active .circle {
        width: calc(70% - 10px);
    }
</style>
