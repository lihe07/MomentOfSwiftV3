<script>
    import Background from "./components/Background.svelte";
    import Card from "./components/Card.svelte";
    import Link from "./reusable/Link.svelte";
    import Router from "svelte-spa-router";

    import Login from "./routes/Login.svelte";
    import Register from "./routes/register.svelte";
    import NotFound from "./routes/404.svelte";

    import Switch from "./reusable/Switch.svelte";
    import { theme } from "./stores";
    import { location } from "svelte-spa-router";
    import { tweened } from "svelte/motion";
    import { cubicInOut } from "svelte/easing";
    import { onMount } from "svelte";

    const routes = {
        "/": Login,
        "/register": Register,
        "/.*": NotFound,
    };

    let inner;
    let outer_height = tweened(0, {
        duration: 300,
        easing: cubicInOut,
    });

    const observer = new ResizeObserver(() => {
        outer_height.set(inner.offsetHeight);
    });
    onMount(() => {
        observer.observe(inner);
    });

    // 检查location有无next属性
    let next = false;

    if (window.location.href.split("?")[1]) {
        let key = window.location.href.split("?")[1].split("=")[0];
        let value = window.location.href.split("?")[1].split("=")[1];
        value = decodeURIComponent(value);
        next = value;
    }

    // setInterval(() => {
    //     console.log(inner_height);
    // }, 100);
</script>

<Background instant={next !== false}>
    <main class:dark={$theme === "dark"} class:light={$theme === "light"}>
        <Card class="main-card">
            <div style:height={$outer_height + "px"} class="outer">
                <div class="inner" bind:this={inner}>
                    <div class="header">
                        <div>
                            <h1 class="title">
                                {#if $location === "/register"}
                                    欢迎新成员加入！
                                {:else if $location === "/" && next === false}
                                    欢迎回来！
                                {:else if next}
                                    先登陆吧！
                                {:else}
                                    404
                                {/if}
                            </h1>
                            <span class="subtitle">
                                {#if $location === "/register" || $location === "/" && next === false}
                                    请登录或注册吧 ヾ(*ﾟ▽ﾟ)ﾉ
                                {:else if next}
                                    访问这个地址需要登录
                                {:else}
                                    Not Found :(
                                {/if}
                            </span>
                        </div>

                        <Switch
                            class="theme-switch"
                            on:change={() => {
                                theme.update((t) =>
                                    t === "dark" ? "light" : "dark"
                                );
                            }}
                            checked={$theme === "dark"}
                        />
                    </div>

                    <Router {routes} />
                </div>
            </div>
        </Card>
    </main>
</Background>

<style>
    main {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        height: 100vh;
    }

    .header {
        display: flex;
        justify-content: space-between;
        align-items: center;
    }

    :global(h1) {
        font-size: 24px;
        font-weight: bold;
        font-family: var(--font-sans-serif);
        letter-spacing: 6px;
        /* 深浅转换 .2s */
        transition: color 0.2s;
    }

    :global(main.dark h1) {
        color: #fff;
    }

    .title {
        margin-top: 0;
        margin-bottom: 1px;
    }

    .subtitle {
        font-size: 14px;
        /* 深浅转换 .2s */
        transition: color 0.2s;
        font-family: var(--font-sans-serif);
        letter-spacing: 1px;
    }

    main.dark .subtitle {
        color: rgba(194, 194, 194, 1);
    }

    main.light .subtitle {
        color: rgba(61, 61, 61, 1);
    }

    main :global(.main-card) {
        width: 100%;
        max-width: 400px;
    }

    main :global(.theme-switch) {
        width: 80px !important;
        height: 25px !important;
    }

    /*一个奇技淫巧 自适应高度变化*/
    .outer {
        overflow: hidden;
        height: 100%;
    }

    .inner {
        height: max-content;
        width: 100%;
    }
</style>
