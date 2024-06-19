import { useLocation, useNavigate } from "@solidjs/router";
import { createQuery } from "@tanstack/solid-query";
import { createEffect } from "solid-js";
import { z } from "zod";
import { ekklesiaApi } from "../ky";

export type User = z.infer<typeof userSchema>;
const userSchema = z.object({
    id: z.string(),
    email: z.string(),
    image: z.string().nullable(),
    name: z.string().nullable(),
});

export const getUserQueryKey = ["get-user"];

export const useUser = () => {
    const navigate = useNavigate();
    const location = useLocation();
    const query = createQuery(() => ({
        queryFn: async () => {
            const res = await ekklesiaApi.get("user", {
                hooks: {
                    afterResponse: [
                        (_input, _options, response) => {
                            if (
                                response.status === 401 &&
                                !["/auth/sign-in", "/auth/sign-up"].includes(location.pathname)
                            ) {
                                navigate("/auth/sign-in", { replace: true });
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

export const createAuthenticationPageGate = () => {
    const user = useUser();

    createEffect(() => {
        if (!user.isLoading && user.data) {
            location.href = "/app";
        }
    });
};

export const createAuthenticatedPageGate = () => {
    const user = useUser();
    createEffect(() => {
        if (!user.isLoading && !user.data) {
            location.href = "/auth/sign-in";
        }
    });
};
