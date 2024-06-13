import { useLocation, useNavigate } from "@solidjs/router";
import { createQuery } from "@tanstack/solid-query";
import { z } from "zod";
import { ekklesiaApi } from "../ky";

export type User = z.infer<typeof userSchema>;
const userSchema = z.object({
    email: z.string(),
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
                            if (response.status === 401 && location.pathname !== "/sign-in") {
                                navigate("/sign-in", { replace: true });
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
