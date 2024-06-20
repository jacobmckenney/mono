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
            const res = await ekklesiaApi.get("auth/link/google");
            const json = await res.json();
            return authUrlSchema.parse(json).url;
        },
        mutationKey: ["get-google-auth-url"],
        onSuccess: (url) => {
            window.location.href = url;
        },
    }));
    return (
        <>
            <Button {...rest} onClick={() => getLink.mutate()}>
                {type} with Microsoft
            </Button>
        </>
    );
};
