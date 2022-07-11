<script>
    import Card from "./Card.svelte";

    import {theme} from "../stores";

    export let id = 114514;
    export let thumbnail_url = "https://raw.githubusercontent.com/lihe07/MomentOfSwiftV3/main/Local/SSOWeb/src/791531a78c9416e709ff627dfc1fbe4.jpg";
    export let num = 123;
    export let creator_id = "";
    export let creator_name = "lihe07";
    export let creator_avatar = "https://avatars.githubusercontent.com/u/53819558?v=4";

</script>

<Card clickable={true} no_padding={true} on:click={() => location.hash = "#/edit/" + id}>
    <div class="container" style="background-image: url({thumbnail_url});" class:dark={$theme === "dark"}
         class:light={$theme === "light"}>
        <div class="info">
            <div style="margin-left: 10px;display: flex;align-items: center;">
                <img alt="avatar of {creator_name}" src={creator_avatar}>
                <span class="name">{creator_name}</span>
            </div>

            <span class="num" style="margin-right: 10px;">{num}</span>
        </div>
    </div>
</Card>


<style>
    .container {
        width: 100%;
        height: 100%;
        border-radius: 20px;
        background-size: cover;
        position: relative;
    }

    .info {
        position: absolute;
        left: 0;
        bottom: 0;
        height: 70px;
        width: 100%;
        border-bottom-left-radius: 20px;
        border-bottom-right-radius: 20px;
        display: flex;
        justify-content: space-between;
        align-items: center;
    }

    .dark .info {
        background-color: rgba(0, 0, 0, .5);
    }

    .light .info {
        background-color: rgba(255, 255, 255, .5);
    }

    .dark {
        color: white;
    }

    .light {
        color: black;
    }

    span {
        font-family: var(--font-sans-serif);
    }

    .name {
        font-size: 20px;
        margin-left: 10px;
        opacity: .7;
    }

    .num {
        font-size: 25px;
        letter-spacing: 2px;
        font-family: var(--font-mono);
    }

    img {
        width: 50px;
        height: 50px;
        border-radius: 50%;
    }
</style>
