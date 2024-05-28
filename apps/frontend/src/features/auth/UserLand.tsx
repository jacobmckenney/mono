import { Accessor, Component, JSX, Match, Switch, createContext, useContext } from "solid-js";
import { User, getUser } from "../../lib/hooks/auth";

const UserContext = createContext<{ user: Accessor<User | null> }>({
    user: () => null,
});

export const UserProvider: Component<{ children: JSX.Element }> = ({ children }) => {
    const user = getUser();
    return <UserContext.Provider value={{ user }}>{children}</UserContext.Provider>;
};

const useContextUser = () => {
    const context = useContext(UserContext);
    if (!context) {
        throw new Error("useUser must be used within a UserProvider");
    }
    return context;
};

export const useUser = () => {
    const { user } = useContextUser();
    return { user };
};

export const UserLand: Component<{ children: JSX.Element }> = ({ children }) => {
    const { user } = useContextUser();
    return (
        <>
            {JSON.stringify({ user: user() })}
            <Switch>
                <Match when={user() !== null}>{children}</Match>
                {/* <Match when={loading()}>loading...</Match> */}
            </Switch>
        </>
    );
};
