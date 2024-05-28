import { createMutation } from "@tanstack/solid-query";
import { Component } from "solid-js";
import { z } from "zod";
import { Button } from "../../components/Button";
import { getUser } from "../../lib/hooks/auth";
import { ekklesiaApi } from "../../lib/ky";

export const authUrlSchema = z.object({
    url: z.string(),
});

interface Props {
    type: "Sign-in" | "Sign-up";
}

export const GoogleAuthButton: Component<Props> = ({ type }) => {
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
    const user = getUser();

    return (
        <>
            <Button disabled={getLink.isPending} onClick={() => getLink.mutate()}>
                {type} with Google
            </Button>
        </>
    );
};
