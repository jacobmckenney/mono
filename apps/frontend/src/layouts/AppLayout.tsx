import { Component, JSX, children } from "solid-js";
import { createAuthenticatedPageGate, useUser } from "../lib/hooks/auth";

interface Props {
    children: JSX.Element;
}

export const AppLayout: Component<Props> = (props) => {
    createAuthenticatedPageGate();
    const c = children(() => props.children);
    return (
        <div>
            <div class="bg-white dark:bg-black h-[calc(100vh_-_40px)] overlow-y-auto">{c()}</div>
            <div class="bg-black dark:bg-white h-10 justify-between flex items-center w-full">
                <h1>Ekklesia</h1>
                <div>
                    <UserProfile />
                </div>
            </div>
        </div>
    );
};

const UserProfile = () => {
    const user = useUser();

    return (
        <div>
            <img src={user.data?.image || ""} alt={user.data?.name ?? ""} class="h-10 w-10 rounded-full" />
        </div>
    );
};
