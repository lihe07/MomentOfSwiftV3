<script>
    import Background from "./components/Background.svelte";
    import Card from "./components/Card.svelte";
    import Link from "./reusable/Link.svelte";
    import Router from "svelte-spa-router";
    import Login from "./routes/Login.svelte";
    import Register from "./routes/register.svelte";
    import Switch from "./reusable/Switch.svelte";
    import {theme} from "./stores";
    import {location} from "svelte-spa-router";
    import {tweened} from "svelte/motion";
    import {cubicInOut} from "svelte/easing";

    const routes = {
        "/": Login,
        "/register": Register,
    };

    let inner_height;
    let outer_height = tweened(0, {
        duration: 300,
        easing: cubicInOut
    });
    let initial = true;
    $: {
        if (inner_height && initial) {
            outer_height.set($location === "/register" ? 253 : 358)
            initial = false;
        } else if (inner_height) {
            outer_height.set(inner_height)
        }
    }
</script>

<Background>
    <main class:dark={$theme === "dark"} class:light={$theme === "light"}>
        <Card class="main-card">
            <div style:height={$outer_height + "px"} class="outer">
                <div class="inner" bind:offsetHeight={inner_height}>
                    <div class="header">
                        <div>
                            <h1 class="title">欢迎{$location === "/register" ? "新成员加入" : "回来"}!</h1>
                            <span class="subtitle">请登录或注册吧 ヾ(*ﾟ▽ﾟ)ﾉ </span>
                        </div>

                        <Switch
                                class="theme-switch"
                                on:change={() => {
                        theme.update((t) => (t === "dark" ? "light" : "dark"));
                    }}
                                checked={$theme === "dark"}
                        />
                    </div>

                    <Router {routes}/>
                    <div style="height: 20px;"></div>

                </div>
            </div>
            <div style="text-align: center;">
                <Link href={$location === "/register" ? "#" : "#/register"}>
                    {$location === "/register" ? "登录" : "注册"}
                </Link>
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
