import { render } from "solid-js/web";

import { QueryClient, QueryClientProvider } from "@tanstack/solid-query";
import { App } from "./App";

const client = new QueryClient({
    defaultOptions: {
        queries: {
            refetchOnMount: true,
            refetchOnWindowFocus: true,
            refetchOnReconnect: true,
        },
    },
});

const root = document.getElementById("root");

render(
    () => (
        <QueryClientProvider client={client}>
            <App />
        </QueryClientProvider>
    ),
    root!
);
