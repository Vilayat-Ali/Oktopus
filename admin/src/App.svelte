<script lang="ts">
    import { Router, Route } from "svelte-routing";
    import RoutingPages from "./pages";
</script>

<Router>
    {#each RoutingPages as routingPage}
        <Route path={routingPage.to} component={routingPage.pageComponent} />
    {/each}
</Router>