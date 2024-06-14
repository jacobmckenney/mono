import { Component, JSX, children } from "solid-js";
import { createAuthenticatedPageGate, useUser } from "../lib/hooks/auth";

interface Props {
    children: JSX.Element;
}

export const AppLayout: Component<Props> = (props) => {
    createAuthenticatedPageGate();
    const c = children(() => props.children);
    const user = useUser();
    return (
        <div>
            <div class="bg-black dark:bg-white h-10"></div>
            <div class="bg-white dark:bg-black min-h-screen">{c()}</div>
        </div>
    );
};

const UserProfile = () => {
    return <div></div>;
};
