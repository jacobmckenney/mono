import { Component, JSX, children, createEffect } from "solid-js";
import { useUser } from "../lib/hooks/auth";

const gateAdminPage = () => {
    const user = useUser();

    createEffect(() => {
        if (!user.isLoading && user.data && !user.data.isAdmin) {
            location.href = "/app";
        }
    });
};

interface Props {
    children: JSX.Element;
}

export const AdminLayout: Component<Props> = (props) => {
    gateAdminPage();
    const c = children(() => props.children);
    return (
        <>
            <div>Admin sidebar{c()}</div>
        </>
    );
};
