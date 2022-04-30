<script>
    // Stores & APIs
    import { theme } from "../stores";
    // Reusable
    import Switch from "../reusable/Switch.svelte";
</script>

<div
    class="main"
    class:dark={$theme === "dark"}
    class:light={$theme === "light"}
>
    <span>LOGO HERE</span>
    <div class="right">
        <a>
            <svg
                xmlns="http://www.w3.org/2000/svg"
                xmlns:xlink="http://www.w3.org/1999/xlink"
                viewBox="0 0 24 24"
                ><g
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    ><circle cx="12" cy="13" r="2" /><path
                        d="M13.45 11.55L15.5 9.5"
                    /><path d="M6.4 20a9 9 0 1 1 11.2 0z" /></g
                ></svg
            >
            控制台
        </a>
        <a>
            <svg
                xmlns="http://www.w3.org/2000/svg"
                xmlns:xlink="http://www.w3.org/1999/xlink"
                viewBox="0 0 16 16"
                ><g fill="none"
                    ><path
                        d="M8 3.5a5.5 5.5 0 0 0-3.89 9.387a.75.75 0 1 1-1.062 1.06a7 7 0 1 1 9.904 0a.75.75 0 1 1-1.061-1.06A5.5 5.5 0 0 0 8 3.5zm0 4a1.5 1.5 0 1 0 0 3a1.5 1.5 0 0 0 0-3zM5.25 9a2.75 2.75 0 1 1 4.696 1.943a.75.75 0 0 0 1.062 1.06a4.25 4.25 0 1 0-6.015 0a.75.75 0 1 0 1.06-1.06A2.739 2.739 0 0 1 5.25 9z"
                        fill="currentColor"
                    /></g
                ></svg
            >
            社区
        </a>
        <a class="active">
            <svg
                xmlns="http://www.w3.org/2000/svg"
                xmlns:xlink="http://www.w3.org/1999/xlink"
                viewBox="0 0 24 24"
                ><g
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    ><path d="M8 8h8v8H8z" /><path d="M4 4v.01" /><path
                        d="M8 4v.01"
                    /><path d="M12 4v.01" /><path d="M16 4v.01" /><path
                        d="M20 4v.01"
                    /><path d="M4 20v.01" /><path d="M8 20v.01" /><path
                        d="M12 20v.01"
                    /><path d="M16 20v.01" /><path d="M20 20v.01" /><path
                        d="M20 16v.01"
                    /><path d="M20 12v.01" /><path d="M20 8v.01" /><path
                        d="M4 16v.01"
                    /><path d="M4 12v.01" /><path d="M4 8v.01" /></g
                ></svg
            >
            检测
        </a>
        <div style="width: 20px;" />
        <Switch
            class="switch"
            on:change={() => {
                theme.update((v) => (v === "dark" ? "light" : "dark"));
            }}
            checked={$theme === "dark"}
        />
    </div>
</div>

<style>
    .main {
        width: 100vw;
        height: 100px;
        display: flex;
        justify-content: space-between;
        align-items: center;
    }
    .main.dark {
        background-color: #212130;
    }
    .main.light {
        background-color: white;
    }
    span {
        margin-left: 40px;
    }
    .right {
        display: flex;
        margin-right: 40px;
    }
    a {
        display: flex;
        width: max-content;
        margin: 0 20px;
        align-items: center;
        justify-content: center;
        letter-spacing: 1px;
        color: #42a5f5;
        opacity: 0.6;
        line-height: 100%;
        font-size: 18px;
        font-family: var(--font-sans-serif);
    }
    a > svg {
        width: 19px;
        margin-right: 10px;
    }

    .dark a {
        color: rgba(204, 231, 237, 1);
    }
    a.active {
        opacity: 1;
    }
    .dark a.active {
        color: white;
        text-shadow: 0px 0px 4px rgba(255, 255, 255, 0.5);
    }
    .light a.active {
        color: black;
        text-shadow: 0px 0px 4px rgba(0, 0, 0, 0.5);
    }
    .light a.active svg {
        filter: drop-shadow(0, 0, 4px, rgba(0, 0, 0, 0.5));
    }
    .main :global(.switch) {
        width: 70px !important;
        height: 25px !important;
    }
</style>
