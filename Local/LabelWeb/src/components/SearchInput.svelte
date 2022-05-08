<script>
    // Stores & APIs
    import {theme} from "../stores";
    import {createEventDispatcher} from "svelte";

    const dispatch = createEventDispatcher();

    // Data
    export let value = "";
    let focused = false;
</script>

<div class="container" class:dark={$theme === "dark"} class:light={$theme === "light"} class:focused>

        <span class="icon">
            <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 512 512"><path
                    d="M456.69 421.39L362.6 327.3a173.81 173.81 0 0 0 34.84-104.58C397.44 126.38 319.06 48 222.72 48S48 126.38 48 222.72s78.38 174.72 174.72 174.72A173.81 173.81 0 0 0 327.3 362.6l94.09 94.09a25 25 0 0 0 35.3-35.3zM97.92 222.72a124.8 124.8 0 1 1 124.8 124.8a124.95 124.95 0 0 1-124.8-124.8z"
                    fill="currentColor"></path></svg>
        </span>
    <input on:keydown={e => {
            if (e.key === "Enter") {
                dispatch("search", value);
            }
        }} bind:value={value} on:focus={() => {focused = true}} on:blur={() => {focused = false}}>
    <span class="clear" class:show={value.length > 0} on:click={() => {value = ""}}>
                <svg version="1.1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" x="0px"
                     y="0px"
                     viewBox="0 0 512 512" enable-background="new 0 0 512 512" xml:space="preserve"><path d="M278.6,256l68.2-68.2c6.2-6.2,6.2-16.4,0-22.6c-6.2-6.2-16.4-6.2-22.6,0L256,233.4l-68.2-68.2c-6.2-6.2-16.4-6.2-22.6,0
            c-3.1,3.1-4.7,7.2-4.7,11.3c0,4.1,1.6,8.2,4.7,11.3l68.2,68.2l-68.2,68.2c-3.1,3.1-4.7,7.2-4.7,11.3c0,4.1,1.6,8.2,4.7,11.3
            c6.2,6.2,16.4,6.2,22.6,0l68.2-68.2l68.2,68.2c6.2,6.2,16.4,6.2,22.6,0c6.2-6.2,6.2-16.4,0-22.6L278.6,256z"></path></svg>
        </span>
</div>


<style>
    * {
        transition: all .2s;
    }
    .container {
        display: flex;
        align-items: center;
        justify-content: center;
        height: 40px;
        width: 210px;
        border-radius: 20px;
        padding: 5px 15px;
        transition: all .2s;
    }

    .light.container {
        background-color: #fff;
        box-shadow: rgba(0, 0, 0, 0.08) 0px 4px 6px 0px, rgba(0, 0, 0, 0.05) 0px 0px 0px 1px;
    }
    .dark.container {
        background-color: #2E2E40;
        box-shadow: rgba(0, 0, 0, 0.3) 0px 4px 6px 0px, rgba(0, 0, 0, 0.2) 0px 0px 0px 1px;
    }


    .container:hover, .container.focused {
        box-shadow: 0 0 0 rgba(0, 0, 0, 0);
    }

    input {
        width: 100%;
        height: 100%;
        border: none;
        outline: none;
        padding: 0;
        background: transparent;
        font-size: 18px;
        margin: 0 5px;
    }

    .light input {
        color: #000;
    }
    .dark input {
        color: #fff;
    }

    .icon {
        display: flex;
        align-items: center;
        justify-content: center;
        width: max-content;
        height: max-content;
    }

    .icon svg {
        width: 18px;
        height: 18px;

    }
    .light svg {
        color: rgba(0, 0, 0, 0.5);
        fill: rgba(0, 0, 0, 0.5);
    }
    .dark svg {
        color: rgba(255, 255, 255, 0.5);
        fill: rgba(255, 255, 255, 0.5);
    }

    .clear {
        display: flex;
        align-items: center;
        justify-content: center;
        cursor: pointer;
    }

    .clear svg {
        width: 1.5rem;
        height: 1.5rem;
    }

    .clear {
        opacity: 0;
        transition: opacity 0.2s;
    }

    .clear.show {
        opacity: 1;
    }

</style>


