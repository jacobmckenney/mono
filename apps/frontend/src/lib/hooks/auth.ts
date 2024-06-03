import { createQuery } from "@tanstack/solid-query";
import { z } from "zod";
import { ekklesiaApi } from "../ky";

// TODO: get this from a graphql schema
const userSchema = z.object({
    email: z.string(),
});

export type User = z.infer<typeof userSchema>;
export const getUserQueryKey = ["get-user"];

export const useUser = () => {
    const query = createQuery(() => ({
        queryFn: async () => {
            const res = await ekklesiaApi.get("user", {
                hooks: {
                    afterResponse: [
                        (_input, _options, response) => {
                            if (response.status === 401 && window.location.pathname !== "/login") {
                                window.location.href = "/login";
                            }
                        },
                    ],
                },
            });
            const json = await res.json();
            const user = userSchema.parse(json);
            return user;
        },
        queryKey: getUserQueryKey,
    }));
    return query;
};
