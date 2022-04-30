<script>
    // Stores & APIs
    import { theme } from "../stores";
    import { fly } from "svelte/transition";
    import { createEventDispatcher } from "svelte";
    // Reusable
    import Input from "../reusable/Input.svelte";
    import Button from "../reusable/Button.svelte";

    export let user;
    const dispatch = createEventDispatcher();
    let password = "";
</script>

<div
    in:fly={{ x: 40 }}
    class:dark={$theme === "dark"}
    class:light={$theme === "light"}
    class="main"
>
    <img src={user.avatar} class="avatar" alt={"Avatar of " + user.name} />
    <h1>{user.name}</h1>
    <Input
        type="password"
        class="password"
        placeholder="密码"
        auto_focus={true}
        bind:value={password}
        on:enter={() => {
            dispatch("next", password);
        }}
    />
    <Button
        class="button"
        on:click={() => {
            dispatch("next", password);
        }}>下一步</Button
    >
</div>

<style>
    .avatar {
        width: 150px;
        height: 150px;
        border-radius: 50%;
    }
    .main {
        display: flex;
        justify-content: center;
        align-items: center;
        flex-direction: column;
        padding: 40px;
    }
    h1 {
        font-size: 28px;
    }
    .main :global(.password) {
        margin: 20px 0;
        width: 360px;
    }
    .main :global(.button) {
        width: 400px;
    }
</style>
