<script>
    import {theme} from "../stores.js";
</script>

<div class="bg">
    <div
            class="mask"
            class:dark={$theme === "dark"}
            class:light={$theme === "light"}
    >
        <p class="likes">
            <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 512 512">
                <path d="M462.3 62.6C407.5 15.9 326 24.3 275.7 76.2L256 96.5l-19.7-20.3C186.1 24.3 104.5 15.9 49.7 62.6c-62.8 53.6-66.1 149.8-9.9 207.9l193.5 199.8c12.5 12.9 32.8 12.9 45.3 0l193.5-199.8c56.3-58.1 53-154.3-9.8-207.9z"
                      fill="currentColor"></path>
            </svg>
            114514
        </p>
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
        font-family: var(--font-sans-serif);
        transition: all .2s;
        letter-spacing: 1px;
    }

    /**/
    .light .copyright {
        color: rgba(0, 0, 0, .2);
    }

    .dark .copyright {
        color: rgba(255, 255, 255, .3);
    }

    .likes {
        margin: 20px;
        position: absolute;
        left: 0;
        bottom: 0;
        font-size: 12px;
        display: flex;
        font-family: var(--font-sans-serif);
        align-items: center;
        transition: all .2s;
    }

    .likes svg {
        width: 12px;
        height: 12px;
        margin-right: 3px;
    }

    .light .likes {
        color: rgba(0, 0, 0, .2);
    }

    .dark .likes {
        color: rgba(255, 255, 255, .3);

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
