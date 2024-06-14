import { useNavigate } from "@solidjs/router";
import { createMutation, useQueryClient } from "@tanstack/solid-query";
import { Component } from "solid-js";
import { Button } from "../../components/Button";
import { getUserQueryKey } from "../../lib/hooks/auth";
import { ekklesiaApi } from "../../lib/ky";

export const LogoutButton: Component = () => {
    const queryClient = useQueryClient();
    const navigate = useNavigate();
    const logoutMutation = createMutation(() => ({
        mutationFn: async () => {
            await ekklesiaApi.post("auth/logout");
        },
        mutationKey: ["logout"],
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: getUserQueryKey });
            navigate("/auth/sign-in");
        },
    }));
    return <Button onClick={async () => await logoutMutation.mutateAsync()}>Logout</Button>;
};
