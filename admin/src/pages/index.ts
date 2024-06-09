// pages
import Account from "./pages/Account.svelte";
import Dashboard from "./pages/Dashboard.svelte";

type RoutingPage = {
    to: string;
    pageComponent: any;
}

const routingPages: RoutingPage[] = [
    {
        to: '/account',
        pageComponent: Account,
    },
    {
        to: '/',
        pageComponent: Dashboard,
    }
];

export default routingPages;