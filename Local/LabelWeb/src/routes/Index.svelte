<script>
    // Stores & APIs
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


    const routes = {
        "/mine": MyTasks,
        "/all": AllTasks,
    }
    console.log($location)
    if ($location === "/") {
        window.location.href = "#/mine";
    }
    $:tab = $location === "/mine" ? 0 : 1;

</script>

<div class:dark={$theme === "dark"} class:light={$theme === "light"}>
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
    .header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 0 20px;
    }
</style>
