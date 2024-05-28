import { Component } from "solid-js";
import { GoogleAuthButton } from "./features/auth/GoogleAuthButton";
import { MicrosoftAuthButton } from "./features/auth/MicrosoftAuthButton";
import { UserLand, UserProvider, useUser } from "./features/auth/UserLand";
import "./index.css";

export const App: Component = () => {
    return (
        <UserProvider>
            <div class=" w-[100vw]">
                <GoogleAuthButton type="Sign-in" />
                <MicrosoftAuthButton type="Sign-in" />
            </div>
            <UserLand>
                <Test />
            </UserLand>
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
            <h1>{user?.email}</h1>
        </div>
    );
};
