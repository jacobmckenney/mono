import { Route } from "@solidjs/router";
import { Component, JSX, children, createEffect } from "solid-js";
import { useUser } from "../lib/hooks/auth";

export const AuthenticateRoute: Component<{ children: JSX.Element; path: string }> = (props) => {
    const c = children(() => props.children);
    return (
        <Route
            path={props.path}
            component={() => (
                <div>
                    <EnforceAuthentication>{c()}</EnforceAuthentication>
                </div>
            )}
        />
    );
};

const EnforceAuthentication: Component<{ children: JSX.Element }> = (props) => {
    const user = useUser();
    const c = children(() => props.children);

    createEffect(() => {
        if (!user.isLoading && user.data) {
            location.href = "/";
        }
    });
    return c();
};
