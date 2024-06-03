import { createMutation, useQueryClient } from "@tanstack/solid-query";
import { Component } from "solid-js";
import { Button } from "../../components/Button";
import { getUserQueryKey } from "../../lib/hooks/auth";
import { ekklesiaApi } from "../../lib/ky";

export const LogoutButton: Component = () => {
    const queryClient = useQueryClient();
    const logoutMutation = createMutation(() => ({
        mutationFn: async () => {
            const res = await ekklesiaApi.post("auth/logout");
        },
        mutationKey: ["logout"],
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: getUserQueryKey });
        },
    }));
    return <Button onClick={async () => await logoutMutation.mutateAsync()}>Logout</Button>;
};
