<script>
    import Background from "./components/Background.svelte";
    import Card from "./components/Card.svelte";
    import Link from "./reusable/Link.svelte";
    import Router from "svelte-spa-router";
    import Login from "./routes/Login.svelte";
    import Register from "./routes/register.svelte";
    import Switch from "./reusable/Switch.svelte";
    import { theme } from "./stores";
    import { location } from "svelte-spa-router";

    const routes = {
        "/": Login,
        "/register": Register,
    };
</script>

<Background>
    <main class:dark={$theme === "dark"} class:light={$theme === "light"}>
        <Card class="main-card">
            <div class="header">
                <div>
                    <h1 class="title">欢迎 {$location}</h1>
                    <span class="subtitle">请登录或注册吧</span>
                </div>

                <Switch
                    class="theme-switch"
                    on:change={() => {
                        theme.update((t) => (t === "dark" ? "light" : "dark"));
                    }}
                    checked={$theme === "dark"}
                />
            </div>

            <Router {routes} />
            <div style="text-align: center;">
                <Link href={$location === "/register" ? "#" : "#/register"}>
                    {$location === "/register" ? "登录" : "注册"}
                </Link>
            </div>
        </Card>
    </main>
</Background>

<style>
    @font-face {
        font-family: "Source Han Sans";
        src: url("/SourceHanSansSC-VF.ttf.woff2");
    }
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
        font-family: "Source Han Sans";
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
        font-family: "Source Han Sans";
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
</style>
