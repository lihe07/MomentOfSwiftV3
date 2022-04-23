<script>
    import {theme} from "../stores.js";
</script>

<div class="bg">
    <div
            class="mask"
            class:dark={$theme === "dark"}
            class:light={$theme === "light"}
    >
        <p class="copyright">图片版权所属....</p>
    </div>
</div>
<div class="content">
    <slot/>
</div>

<style>
    @keyframes blur {
        from {
            opacity: 0;
            filter: blur(20px);
        }
        to {
            opacity: 1;
            filter: blur(0);
        }
    }

    @keyframes fade {
        from {
            opacity: 0;
        }
        to {
            opacity: 1;
        }
    }

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
        z-index: -1;
        /* 深浅转换 .2s */
        transition: background-color 0.2s;
        /*入场动画*/

        opacity: 0;
        animation: fade 1.5s .5s forwards;

    }

    .mask.dark {
        background: rgba(0, 0, 0, 0.8);
    }

    .mask.light {
        background: rgba(255, 255, 255, 0.6);
    }
    .copyright {
        font-size: 12px;
        margin: 20px;
        position: absolute;
        right: 0;
        bottom: 0;
    }
    .light .copyright {
        color: rgba(0, 0, 0, .2);
    }
    .dark .copyright {
        color: rgba(255, 255,255,.3);
    }

    .content {
        position: absolute;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        z-index: 1;

        opacity: 0;
        filter: blur(20px);

        animation: blur 1s 1s forwards;
    }
</style>
