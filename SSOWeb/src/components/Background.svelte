<script>
    import { theme } from "../stores.js";
</script>

<div class="bg">
    <div
        class="mask"
        class:dark={$theme === "dark"}
        class:light={$theme === "light"}
    />
</div>
<div class="content">
    <slot />
</div>

<style>
    .bg {
        background: url("/bg.jpg");
        background-size: cover;
        position: absolute;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        z-index: -1;
    }
    .mask {
        position: absolute;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        backdrop-filter: blur(10px);
        z-index: -1;
        /* 深浅转换 .2s */
        transition: background-color 0.2s;
    }
    .mask.dark {
        background: rgba(0, 0, 0, 0.8);
    }
    .mask.light {
        background: rgba(255, 255, 255, 0.6);
    }
    .content {
        position: absolute;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        z-index: 1;
    }
</style>
