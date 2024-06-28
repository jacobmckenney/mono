import { Component, JSX, Match, Switch, children } from "solid-js";
import { Spinner } from "../components/Spinner";
import { createAuthenticatedPageGate, useUser } from "../lib/hooks/auth";

interface Props {
    children: JSX.Element;
}

export const AppLayout: Component<Props> = (props) => {
    const user = useUser();
    createAuthenticatedPageGate();
    const c = children(() => props.children);
    return (
        <div>
            <div class="bg-white dark:bg-black h-[calc(100dvh_-_40px)] overlow-y-auto">
                <Switch>
                    <Match when={user.isLoading || !user.data}>
                        <div class="h-full w-full text-white items-center justify-center flex">
                            <Spinner />
                        </div>
                    </Match>
                    <Match when={user.data}>{c()}</Match>
                </Switch>
            </div>
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
