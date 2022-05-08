<script>
    // External
    import Router from "svelte-spa-router";

    // Components
    import Header from "./components/Header.svelte";

    // Stores & APIs
    import {theme} from "./stores";

    // Routes
    import Index from "./routes/Index.svelte";
    import Editor from "./routes/Editor.svelte";

    const routes = {
        "/": Index,
        "/editor": Editor,
        "/*": Index
    };
</script>

<div
        class="container"
        class:dark={$theme === "dark"}
        class:light={$theme === "light"}
>
    <Header/>
    <main>
        <Router {routes}/>
    </main>
</div>

<style>
    * {
        transition: background-color .2s, color .2s, border-color .2s;
    }

    .container {
        width: 100vw;
        height: 100%;
        min-height: calc(100vh - 100px);
    }

    .container.light {
        background-color: #efefef;
    }

    .container.dark {
        background-color: #17171e;
    }

    main {
        padding: 40px;
        max-width: 80rem;
        margin: auto;
    }
</style>
