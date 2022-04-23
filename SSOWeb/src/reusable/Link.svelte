<script>
    import {theme} from "../stores.js";

    export let href;
    export let cd = 200; // 点击延迟
    let cooling = false;
    let dom;

</script>

<a class:dark={$theme === "dark"} class:light={$theme === "light"} {href} bind:this={dom} on:click={e => {
    if (cooling) {
        e.preventDefault()
    } else {
        cooling = true;
        setTimeout(() => {
            cooling = false
            // 让自己失焦
            dom.blur()
        }, cd)
    }
}}>
    <slot/>
</a>
<style>
    a {
        text-decoration: none;
        font-size: 16px;
        font-weight: lighter;
        transition: all 0.2s;
        font-family: var(--font-sans-serif);
        letter-spacing: 2px;
        height: 22px;
        display: inline-block;
        line-height: 22px;
        text-align: center;
    }

    .dark {
        color: rgba(184, 231, 255, 1);
    }

    .light {
        color: rgba(72, 179, 232, 1);
    }

    .dark:hover,
    .dark:focus {
        filter: brightness(0.7);
    }

    .light:hover,
    .light:focus {
        filter: brightness(0.8);
    }

    a:focus {
        outline: none;
    }

    a:active {
        font-size: 14px;
    }
</style>
