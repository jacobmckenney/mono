import { Component } from "solid-js";
import { GoogleAuthButton } from "./features/auth/GoogleAuthButton";
import { MicrosoftAuthButton } from "./features/auth/MicrosoftAuthButton";
import { UserProvider, useUser } from "./features/auth/UserLand";
import "./index.css";
import { getUser } from "./lib/hooks/auth";

export const App: Component = () => {
    const user = getUser();
    return (
        <UserProvider>
            {JSON.stringify(user())}
            <div class=" w-[100vw]">
                <GoogleAuthButton type="Sign-in" />
                <MicrosoftAuthButton type="Sign-in" />
            </div>
        </UserProvider>
    );
};

const Test: Component = () => {
    const { user } = useUser();

    if (!user) {
        return <div>loading...</div>;
    }

    return (
        <div>
            {JSON.stringify({ user })}
            <h1>{user?.name}</h1>
        </div>
    );
};
