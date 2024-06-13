import { Component, JSX, children } from "solid-js";
import { createAuthenticatedPageGate } from "../lib/hooks/auth";

interface Props {
    children: JSX.Element;
}

export const AppLayout: Component<Props> = (props) => {
    createAuthenticatedPageGate();
    const c = children(() => props.children);
    return <>{c()}</>;
};
