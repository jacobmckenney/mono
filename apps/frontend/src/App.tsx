import { Component } from "solid-js";
import { GoogleAuthButton } from "./features/auth/GoogleAuthButton";
// import { LogoutButton } from "./features/auth/LogoutButton";
import { MicrosoftAuthButton } from "./features/auth/MicrosoftAuthButton";
// import { UserProvider } from "./features/auth/UserLand";
import { LogoutButton } from "./features/auth/LogoutButton";
import "./index.css";
import { useUser } from "./lib/hooks/auth";

export const App: Component = () => {
    const user = useUser();
    return (
        // <UserProvider>
        <div class=" w-[100vw]">
            <div class="flex items-center gap-4">
                <GoogleAuthButton type="Sign-in" />
                <MicrosoftAuthButton type="Sign-in" />
                <LogoutButton />
                {JSON.stringify(user.data)}
                <button onClick={() => console.log(user.data?.email)}>print</button>
            </div>
        </div>
        // </UserProvider>
    );
};
