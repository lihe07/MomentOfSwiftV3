<script>
    // Stores & APIs
    import {theme} from "../stores";
    import {createEventDispatcher} from "svelte";
    import {tweened} from "svelte/motion";
    import {cubicInOut} from "svelte/easing";

    const dispatch = createEventDispatcher();

    export let current = 0;
    let margin_left = tweened(current === 0 ? 0 : 120, {duration: 100, easing: cubicInOut});
    let cooling_down = false;

    // Methods
    function cool_down() {
        cooling_down = true;
        setTimeout(() => {
            cooling_down = false;
        }, 1000);

    }
</script>


<div class="container" class:dark={$theme === "dark"} class:light={$theme === "light"} class:cooling_down>
    <div class="focus" style="margin-left: {$margin_left}px">

    </div>
    <div class="tab first" class:current={current === 0} on:click={() => {
        if (cooling_down || current === 0) return;
        current = 0;
        $margin_left = 0
        dispatch("switch", current);
        cool_down();
    }}>
        我的计数
    </div>
    <div class="tab second" class:current={current === 1} on:click={() => {
        if (cooling_down || current === 1) return;
        current = 1;
        $margin_left = 120
        dispatch("switch", current);
        cool_down();
    }}>
        全站计数
    </div>
</div>


<style>
    * {
        transition: all .2s;
    }

    .container {
        display: flex;
        border-radius: 20px;
        height: 50px;
        align-items: center;
    }

    .light.container {
        background-color: #fff;
        box-shadow: rgba(0, 0, 0, 0.08) 0px 4px 6px 0px, rgba(0, 0, 0, 0.05) 0px 0px 0px 1px;
    }

    .dark.container {
        background-color: #2E2E40;
        box-shadow: rgba(0, 0, 0, 0.3) 0px 4px 6px 0px, rgba(0, 0, 0, 0.2) 0px 0px 0px 1px;
    }

    .focus {
        position: absolute;
        height: 50px;
        width: 120px;
        z-index: 0;
        border-radius: 20px;
    }

    .light .focus {
        background: #42A5F5;
        opacity: 0.6;
    }

    .dark .focus {
        background: #42A5F5;
        opacity: 0.7;
    }


    .tab {
        z-index: 1;
        margin: 0 10px;
        width: 100px;
        text-align: center;
        font-family: var(--font-sans-serif);
        cursor: pointer;
        opacity: 0.7;
        letter-spacing: 2px;
        font-size: 15px;
        user-select: none;
    }

    .dark .tab {
        color: #fff;
    }

    .tab.current {
        opacity: 1;
        cursor: default;
    }

    .tab:active {
        scale: 0.9;
    }

    .tab.current:active {
        scale: 1;
    }

    .cooling_down {
        opacity: 0.6;
        box-shadow: 0 0 0 rgba(0, 0, 0, 0) !important;
    }

    .cooling_down .tab {
        cursor: not-allowed;
    }

    .cooling_down .tab:active {
        scale: 1;
    }
</style>
