import { A } from "@solidjs/router";
import { Component } from "solid-js";
import { GoogleAuthButton } from "../features/auth/GoogleAuthButton";
import { MicrosoftAuthButton } from "../features/auth/MicrosoftAuthButton";
import { createAuthenticationPageGate } from "../lib/hooks/auth";

interface Props {
    type: "Sign-in" | "Sign-up";
}

const pageInfo = {
    "Sign-in": {
        other: {
            href: "sign-up",
            title: "Sign Up",
        },
        title: "Sign In",
    },
    "Sign-up": {
        other: {
            href: "sign-in",
            title: "Sign In",
        },
        title: "Sign Up",
    },
} as const;

export const AuthPage: Component<Props> = ({ type }) => {
    createAuthenticationPageGate();
    const info = pageInfo[type];
    return (
        <>
            <div class="w-[100vw] flex flex-row h-[100vh]">
                <div class="relative w-1/2 bg-white max-lg:hidden">
                    <div class="absolute top-1/3 right-1/2 translate-x-1/2 flex flex-col items-center gap-3">
                        <h1 class="text-5xl">Ekklesia</h1>
                        <p class="text-center">
                            A modern bible study tool rooted in <span class="italic">community</span>.
                        </p>
                    </div>
                    <hr />
                </div>
                <div class="w-1/2 bg-black relative max-lg:w-full">
                    <h1 class="relative top-10 left-10 text-4xl text-white lg:hidden">Ekklesia</h1>
                    <div class="absolute w-72 top-1/3 right-1/2 translate-x-1/2 flex flex-col gap-6">
                        <h2 class="text-2xl text-white">{info.title}</h2>
                        {/* TODO: support auth with email */}
                        <div class="flex flex-col gap-0.5 w-full">
                            <label for={`email-${type}`} class="text-white">
                                Email
                            </label>
                            <input id={`email-${type}`} type="email" />
                        </div>
                        <div class="w-full flex gap-1.5 items-center justify-center">
                            <hr class="w-1/2 border-[0.5px] border-white" />
                            <p class="text-white">or</p>
                            <hr class="w-1/2 border-[0.5px] border-white" />
                        </div>
                        <div class="flex items-center gap-3"></div>
                        <div class="flex gap-10 items-center">
                            <GoogleAuthButton type={type} />
                            <MicrosoftAuthButton type={type} />
                        </div>
                        <div class="text-white text-xs">
                            Don't have an account? <A href={`/${info.other.href}`}>{info.other.title}</A>
                        </div>
                    </div>
                </div>
            </div>
        </>
    );
};
