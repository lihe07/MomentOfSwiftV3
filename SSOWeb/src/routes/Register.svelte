<script>
    import Input from "../reusable/Input.svelte";
    import Button from "../reusable/Button.svelte";
    import Link from "../reusable/Link.svelte";
    import { fade } from "svelte/transition";

    let code;
</script>

<div in:fade={{ delay: 300 }} class="main">
    <Input class="input" placeholder="邀请码" bind:value={code} />
    <Button class="button">下一步</Button>
    <div style="height: 10px;" />
    <Link href="#/">登录</Link>
</div>

<style>
    .main :global(.input) {
        margin-top: 40px;
        margin-bottom: 0;
        width: 360px;
    }

    .main :global(.button) {
        width: 100%;
        margin: 20px 0;
    }

    .main {
        text-align: center;
    }
</style>
