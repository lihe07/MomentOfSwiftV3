<script>
    // Stores & APIs
    import {theme} from "../stores";

    export let clickable = false;
    export let no_padding = false;
</script>

<div
        class="main"
        class:dark={$theme === "dark"}
        class:light={$theme === "light"}
        class:clickable
        class:no_padding
        tabindex={clickable ? "0" : undefined}
        on:click
>
    <slot/>
</div>

<style>
    .main {
        padding: 40px;
        border-radius: 20px;
        transition: all 0.2s;
    }

    .dark.main {
        background-color: #212130;
    }

    .light.main {
        background-color: white;
    }

    .clickable {
        cursor: pointer;
    }

    .clickable.light {
        box-shadow: rgba(0, 0, 0, 0.08) 0px 4px 6px 0px, rgba(0, 0, 0, 0.05) 0px 0px 0px 1px;
        border: 3px solid #fff;

    }

    .clickable.dark {
        border: 3px solid #212130;
        box-shadow: rgba(0, 0, 0, 0.3) 0px 4px 6px 0px, rgba(0, 0, 0, 0.2) 0px 0px 0px 1px;
    }

    .clickable:hover {
        box-shadow: 0 0 0 rgba(0, 0, 0, 0);
    }

    .clickable:focus, .clickable:active {
        outline: none;
        border: rgba(66, 165, 245, .6) 3px solid;

    }
    .no_padding {
        padding: 0;
    }
</style>
