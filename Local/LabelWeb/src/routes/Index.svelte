<script>
    // Stores & APIs
    import { theme } from "../stores";
    // Components
    import MyTasks from "../components/MyTasks.svelte";
    // Reusable
    import Card from "../reusable/Card.svelte";
</script>

<div class:dark={$theme === "dark"} class:light={$theme === "light"}>
    <h1>我的计数</h1>
    <hr />
    <select>
        <option>近7天</option>
        <option>近14天</option>
        <option>近30天</option>
        <option>自定义</option>
    </select>

    <input type="date" />
    <span>-</span>
    <input type="date" />

    <h1>最新创建</h1>
    <hr />
</div>

<style>
    h1 {
        letter-spacing: 2px;
        margin-bottom: 10px;
    }
    .dark h1 {
        color: white;
    }
    hr {
        border-style: solid;
        border-width: 2px;
    }
    .dark hr {
        border-color: #2e2e40;
    }
    .light hr {
        border-color: #e7e7e7;
    }
</style>
