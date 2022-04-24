<script>
    import { theme } from "../stores.js";
    export let logs = [];
</script>

<div
    class="main"
    class:dark={$theme === "dark"}
    class:light={$theme === "light"}
>
    <div class="loader" />

    <div class="logs">
        {#each logs.slice(-3) as log}
            <p class="log">
                [<span class={log.type}>{log.type.toUpperCase()}</span>] {log.message}
            </p>
        {/each}
    </div>
</div>

<style>
    .main {
        display: flex;
        flex-direction: column;
        align-items: center;
    }
    .loader {
        --height-of-loader: 4px;
        --loader-color: #42a5f5;
        width: 60%;
        height: var(--height-of-loader);
        border-radius: 30px;
        background-color: rgba(0, 0, 0, 0.2);
        position: relative;
        transition: all 0.2s;
    }
    .dark .loader {
        background-color: rgba(255, 255, 255, 0.2);
    }
    .light .loader {
        background-color: rgba(0, 0, 0, 0.2);
    }
    .loader::before {
        content: "";
        position: absolute;
        background: var(--loader-color);
        top: 0;
        left: 0;
        width: 0%;
        height: 100%;
        border-radius: 30px;
        animation: moving 1s ease-in-out infinite;
    }

    @keyframes moving {
        50% {
            width: 100%;
        }

        100% {
            width: 0;
            right: 0;
            left: unset;
        }
    }
    .logs {
        margin-top: 20px;
    }
    .log {
        margin: 5px 0;
        width: 240px;
        text-align: left;
        font-family: var(--font-sans-serif);
        letter-spacing: 1px;
        transition: color 0.2s;
    }
    .log span {
        transition: color 0.2s;
        font-weight: bold;
        letter-spacing: 1px;
    }
    .dark .log {
        color: rgba(255, 255, 255, 0.3);
    }
    .dark .info {
        color: rgba(255, 255, 255, 0.6);
    }
    .dark .success {
        color: rgba(102, 187, 106, 0.6);
    }
    .dark .warn {
        color: rgba(255, 202, 40, 0.6);
    }
    .dark .error {
        color: rgba(239, 83, 80, 0.6);
    }

    .light .log {
        color: rgba(0, 0, 0, 0.3);
    }
    .light .info {
        color: rgba(0, 0, 0, 0.9);
    }
    .light .success {
        color: rgba(102, 187, 106, 0.9);
    }
    .light .warn {
        color: rgba(255, 202, 40, 0.9);
    }
    .light .error {
        color: rgba(239, 83, 80, 0.9);
    }
</style>
