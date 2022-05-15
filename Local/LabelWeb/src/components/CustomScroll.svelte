<script>
    // Stores & Apis
    import {theme} from "../stores";

    let content_width;
    let content_height;

    let container_width;
    let container_height;


    $:right_height = container_height * (container_height / (content_height - 5)) - 4;
    $:right_top = 102 + container_height * (y_scroll / (content_height - 5));
    let y_scroll;
    let x_scroll;

    $:bottom_width = container_width * (container_width / content_width) - 4;
    $:bottom_right = container_width * (x_scroll / content_width) + 2;
    let loaded = false;
    $: {
        if (content_width && content_height && container_width && container_height) {
            loaded = true;
        }
    }

</script>

<div class="container" bind:offsetWidth={container_width} bind:offsetHeight={container_height}
     class:dark={$theme === "dark"} class:light={$theme === "light"}>
    <div class="do-scroll" on:scroll={e => {
        y_scroll = e.target.scrollTop;
        x_scroll = e.target.scrollLeft;
    }}>
        <div class="content" bind:offsetWidth={content_width} bind:offsetHeight={content_height}>
            <slot></slot>
        </div>

    </div>
    <div class="rail right-rail" style:height={right_height + "px"} style:top={right_top + "px"} class:loaded></div>
    <div class="rail bottom-rail" style:width={bottom_width + "px"} style:left={bottom_right +"px"} class:loaded></div>
</div>


<style>
    .rail {
        opacity: 0;
        border-radius: 5px;
    }
    .dark .rail {
        background-color: rgba(33, 33, 48, 0.5);
    }
    .light .rail {
        background-color: rgba(0, 0, 0, 0.5);
    }

    .rail.loaded {
        opacity: 1;
    }

    .right-rail {
        position: fixed;
        right: 2px;
        width: 5px;
        height: 0;
    }

    .bottom-rail {
        position: fixed;
        bottom: 2px;
        width: 0;
        height: 5px;
    }

    .container {
        position: relative;
        width: 100%;
        height: 100%;
        overflow: hidden;
        z-index: auto;
    }

    .do-scroll {
        overflow: scroll;
        width: 100%;
        height: calc(100% + 5px);
        max-height: inherit;
        scrollbar-width: none;
    }

    .content {
        height: max-content;
        width: max-content;
        min-width: 100%;
    }

</style>
