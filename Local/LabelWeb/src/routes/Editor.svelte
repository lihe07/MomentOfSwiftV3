<!-- 主功能: 编辑器 -->

<script>
    // Stores & Apis
    import {theme} from "../stores";
    import {onMount} from "svelte";

    // Components
    import CustomScroll from "../components/CustomScroll.svelte";
    import EditorToolsPanel from "../components/EditorToolsPanel.svelte";

    export let params;
    alert(params.id)


    // Debug
    let image_url = "https://pic-1301810764.cos.ap-beijing.myqcloud.com/picgo/ffe005fa05e775f18f2f7aad935f4d4.jpg";
</script>

<div class="container" class:dark={$theme === "dark"} class:light={$theme === "light"}>
    <EditorToolsPanel/>
    <CustomScroll>
        <img src={image_url}/>
    </CustomScroll>

</div>


<style>
    .container {
        width: 100vw;
        height: calc(100vh - 100px);
        overflow: hidden;
    }
</style>
