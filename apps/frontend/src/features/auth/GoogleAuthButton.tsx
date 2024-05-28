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
                method: "GET",
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
    const getBruh = createMutation(() => ({
        mutationFn: async () => {
            const test = await fetch("http://localhost:8080/bruh", {
                method: "GET",
                headers: {
                    "Content-Type": "application/json",
                },
            });
            if (test.redirected) {
                window.location.href = test.url;
            }
        },
        mutationKey: ["get-bruh"],
        onSuccess: () => {
            console.log("Bruh");
        },
        onError: (err) => {
            console.error(err);
        },
    }));
    return (
        <>
            <Button disabled={getLink.isPending} onClick={() => getLink.mutate()}>
                {type} with Google
            </Button>
            <Button disabled={getBruh.isPending} onClick={() => getBruh.mutate()}>
                Get bruh test
            </Button>
        </>
    );
};
