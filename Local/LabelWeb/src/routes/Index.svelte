<script>
    // Stores & APIs
    import {onDestroy, onMount} from "svelte";
    import {theme} from "../stores";
    import Router, {location, querystring} from "svelte-spa-router";

    // Components
    import SearchInput from "../components/SearchInput.svelte";
    import TabSwitcher from "../components/TabSwitcher.svelte";
    import SortMethodSelector from "../components/SortMethodSelector.svelte";

    // Reusable
    import Card from "../reusable/Card.svelte";

    // Routes
    import MyTasks from "./MyTasks.svelte";
    import AllTasks from "./AllTasks.svelte";


    export let params;
    const routes = {
        "/mine": MyTasks,
        "/all": AllTasks,
    }
    if ($location === "/") {
        window.location.href = "#/mine";
    }
    $:tab = $location === "/mine" ? 0 : 1;

    function handle_drop(e) {
        e.preventDefault();

        console.log(e)
        dragging_over = false;

    }

    let dragging_over = false;
    let unsupported_msg = "";

    function drag_enter(e) {
        e.preventDefault();
        e.stopPropagation();
        e.dataTransfer.dropEffect = "copy";
        dragging_over = true;
        unsupported_msg = "";
        if (e.dataTransfer.items.length > 1) {
            unsupported_msg = "仅支持单次上传一个图片文件";
            return;
        }
        if (e.dataTransfer.items.length === 0) {
            unsupported_msg = "无法读取文件";
            return;
        }
        let item = e.dataTransfer.items[0];
        if (item.kind === "file") {
            let type = item.type;
            if (type.indexOf("image") === -1) {
                unsupported_msg = "仅支持图片文件";
                return;
            }
        } else {
            unsupported_msg = "仅支持图片文件";
            return;
        }
        unsupported_msg = "";
    }

    function drag_leave(e) {
        // Check mouse position to know if we left the window
        let x = e.clientX, y = e.clientY;
        let w = window.innerWidth, h = window.innerHeight;
        if (x <= 0 || x >= w || y <= 0 || y >= h) {
            // Real leave event
            e.preventDefault();
            e.stopPropagation();
            dragging_over = false;

        }
    }

    function drag_over(e) {
        e.preventDefault();
        e.stopPropagation();
    }


    onMount(() => {
        window.addEventListener("dragenter", drag_enter);
        window.addEventListener("dragleave", drag_leave);
        window.addEventListener("dragover", drag_over);
        window.addEventListener("drop", handle_drop);
        return () => {
            window.removeEventListener("dragenter", drag_enter);
            window.removeEventListener("dragleave", drag_leave);
            window.removeEventListener("dragover", drag_over);
            window.removeEventListener("drop", handle_drop);
        }
    })
</script>


<div class:dark={$theme === "dark"} class:light={$theme === "light"} class="index">
    <div class="drag" class:show={dragging_over}>
        <div class="dashed-box">
            <h2>将图片拖拽到这里快速创建计数</h2>
            <p>支持上传单张图片文件</p>
            <p class="warn">{unsupported_msg}</p>
        </div>
    </div>
    <div class="header">
        <div>
            <SortMethodSelector/>
        </div>
        <div>
            <TabSwitcher current={tab} on:switch={e => {
                window.location.href = `#/${e.detail === 0 ? "mine" : "all"}`;
            }}/>
        </div>
        <div>
            <SearchInput/>
        </div>
    </div>
    <Router {routes}/>

</div>

<style>
    .index {
        max-width: 80rem;
        margin: 40px auto;
    }

    .header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 0 20px;

    }

    .drag {
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        background: rgba(0, 0, 0, 0.5);
        z-index: 100;
        opacity: 0;
        display: flex;
        justify-content: center;
        align-items: center;
        transition: opacity 0.3s ease-in-out;
        pointer-events: none;
    }

    .drag.show {
        opacity: 1;
    }

    .dashed-box {
        margin: 100px;
        padding: 50px;
        border: 1px dashed #fff;
        border-radius: 5px;
        text-align: center;
    }

    h2 {
        color: white;
    }

    p {
        color: white;
    }

    .warn {
        color: #fbc02d;
    }
</style>
