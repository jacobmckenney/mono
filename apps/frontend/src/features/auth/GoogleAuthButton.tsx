import { createMutation } from "@tanstack/solid-query";
import { Component } from "solid-js";
import { z } from "zod";
import { Button, ButtonProps } from "../../components/Button";
import { ekklesiaApi } from "../../lib/ky";

export const authUrlSchema = z.object({
    url: z.string(),
});

interface Props extends ButtonProps {
    type: "Sign-in" | "Sign-up";
}

export const GoogleAuthButton: Component<Props> = ({ type, ...rest }) => {
    // TODO: factor into "Auth Button" and use for Google + Microsoft, etc.
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
            <Button {...rest} disabled={getLink.isPending} onClick={() => getLink.mutate()}>
                {type} with Google
            </Button>
        </>
    );
};
