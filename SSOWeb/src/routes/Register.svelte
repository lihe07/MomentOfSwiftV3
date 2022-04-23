<script>
    import Input from "../reusable/Input.svelte";
    import Button from "../reusable/Button.svelte";
    import { fade } from "svelte/transition";

    let code;
</script>

<div in:fade={{delay: 300}} class="main">
    <Input class="input" placeholder="邀请码" bind:value={code}/>
    <Button class="button">下一步</Button>
</div>

<style>
    .main :global(.input) {
        margin-top: 40px;
        margin-bottom: 0;
        width: 360px;

    }
    .main :global(.button) {
        width: 100%;
        margin-top: 20px;
        margin-bottom: 40px;
    }
    .main {
        text-align: center;
    }
</style>
