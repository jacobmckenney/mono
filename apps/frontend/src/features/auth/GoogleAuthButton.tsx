import { createMutation } from "@tanstack/solid-query";
import { Component } from "solid-js";
import { z } from "zod";
import { Button } from "../../components/Button";

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
            const res = await fetch("http://localhost:8080/auth/link/google", {
                headers: {
                    "Content-Type": "application/json",
                },
            });
            if (!res.ok) throw new Error("Failed to fetch URL");
            const json = await res.json();
            return authUrlSchema.parse(json).url;
        },
        mutationKey: ["get-google-auth-url"],
        onSuccess: (url) => {
            console.log(url);
            window.location.href = url;
        },
    }));
    return (
        <>
            <Button onClick={() => getLink.mutate()}>{type} with Google</Button>
        </>
    );
};
