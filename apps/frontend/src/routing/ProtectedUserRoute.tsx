import { Route } from "@solidjs/router";
import { Component, JSX, Match, Switch, children, createEffect } from "solid-js";
import { useUser } from "../lib/hooks/auth";

export const ProtectedUserRoute: Component<{ children: JSX.Element; path: string }> = (props) => {
    const c = children(() => props.children);
    return <Route path={props.path} component={() => <UserLand>{c()}</UserLand>} />;
};

const UserLand: Component<{ children: JSX.Element }> = (props) => {
    const user = useUser();
    const c = children(() => props.children);

    createEffect(() => {
        if (!user.isLoading && !user.data) {
            location.href = "/auth/sign-in";
        }
    });
    return (
        <Switch>
            <Match when={user?.data}>{c()}</Match>
            <Match when={user?.isLoading}>loading...</Match>
        </Switch>
    );
};
