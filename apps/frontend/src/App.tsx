import { RouteDefinition, Router } from "@solidjs/router";
import { Component } from "solid-js";
import { AuthPage } from "./features/auth/AuthPage";
import { LogoutButton } from "./features/auth/LogoutButton";
import "./index.css";
import { AppLayout } from "./layouts/AppLayout";
import { useUser } from "./lib/hooks/auth";

export const appRoutes = [
    {
        path: "/auth",
        children: [
            {
                path: "/sign-in",
                component: () => <AuthPage type="Sign-in" />,
            },
            {
                path: "/sign-up",
                component: () => <AuthPage type="Sign-up" />,
            },
        ],
    },
    {
        path: ["/", "/app"],
        component: ({ children }) => <AppLayout>{children}</AppLayout>,
        children: [
            {
                path: "/",
                component: () => (
                    <div>
                        <LogoutButton />
                        <Test />
                    </div>
                ),
            },
            {
                path: "/bruh",
                component: () => <div>bruh</div>,
            },
        ],
    },
    {
        path: "*404",
        component: () => <div>404 Not found</div>,
    },
] satisfies Array<RouteDefinition>;

export const App: Component = () => {
    return <Router>{appRoutes}</Router>;
};

const Test: Component = () => {
    const user = useUser();
    return (
        <>
            <div class="text-white">hello {user.data?.name}</div>
        </>
    );
};
