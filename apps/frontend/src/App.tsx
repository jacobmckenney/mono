import { Route, Router } from "@solidjs/router";
import { Component } from "solid-js";
import { LogoutButton } from "./features/auth/LogoutButton";
import "./index.css";
import { SignInPage } from "./pages/SignIn";
import { SignUpPage } from "./pages/SignUp";
import { AuthenticateRoute } from "./routing/AuthenticateRoute";
import { ProtectedUserRoute } from "./routing/ProtectedUserRoute";

const authRoutes = ["sign-in", "sign-up"];

export const App: Component = () => {
    return (
        <Router>
            <AuthenticateRoute path="/sign-in">
                <SignInPage />
            </AuthenticateRoute>
            <Route path="/sign-up" component={() => <SignUpPage />} />
            <ProtectedUserRoute path="/">
                <div class="bg-black">
                    hello world <LogoutButton />
                </div>
            </ProtectedUserRoute>
            <Route path="*404" component={() => <div>404 Not found</div>} />
        </Router>
    );
};
