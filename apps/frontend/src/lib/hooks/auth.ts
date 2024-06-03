import { createQuery } from "@tanstack/solid-query";
import { createSignal } from "solid-js";
import { z } from "zod";
import { ekklesiaApi } from "../ky";

// TODO: get this from a graphql schema
const userSchema = z.object({
    email: z.string(),
});

export type User = z.infer<typeof userSchema>;

export const getUser = () => {
    let [user, setUser] = createSignal<User | null>(null);
    createQuery(() => ({
        queryFn: async () => {
            const res = await ekklesiaApi.get("user");
            console.log(res.ok, res.status, res.statusText, res.url);
            if (res.ok) {
                const json = await res.json();
                const user = userSchema.parse(json);
                setUser(user);
                return user;
            } else {
                setUser(null);
                return null;
            }
        },
        queryKey: ["get-user"],
    }));
    return user;
};
