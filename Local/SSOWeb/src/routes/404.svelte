<script>
    import Error from "../reusable/Error.svelte";
</script>

<Error
    reason="走错门了吧（"
    back_text="去登录"
    on:click={() => {
        location.href = "#/";
    }}
/>
