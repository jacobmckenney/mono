import { createMutation } from "@tanstack/solid-query";
import { Component } from "solid-js";
import { Button, ButtonProps } from "../../components/Button";
import { ekklesiaApi } from "../../lib/ky";
import { authUrlSchema } from "./GoogleAuthButton";

interface Props extends ButtonProps {
    type: "Sign-in" | "Sign-up";
}

export const MicrosoftAuthButton: Component<Props> = ({ type, ...rest }) => {
    const getLink = createMutation(() => ({
        mutationFn: async () => {
            const res = await ekklesiaApi.get("auth/link/microsoft");
            const json = await res.json();
            return authUrlSchema.parse(json).url;
        },
        mutationKey: ["get-microsoft-auth-url"],
        onSuccess: (url) => {
            window.location.href = url;
        },
    }));
    return (
        <>
            <Button
                class="flex gap-3 items-center w-full min-w-fit justify-center h-7"
                {...rest}
                onClick={() => getLink.mutate()}
            >
                <Logo />
                <p>{type} with Microsoft</p>
            </Button>
        </>
    );
};

const Logo = () => (
    <svg xmlns="http://www.w3.org/2000/svg" width="21" height="21" viewBox="0 0 21 21">
        <title>MS-SymbolLockup</title>
        <rect x="1" y="1" width="9" height="9" fill="#f25022" />
        <rect x="1" y="11" width="9" height="9" fill="#00a4ef" />
        <rect x="11" y="1" width="9" height="9" fill="#7fba00" />
        <rect x="11" y="11" width="9" height="9" fill="#ffb900" />
    </svg>
);
