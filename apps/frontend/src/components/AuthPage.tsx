import { A } from "@solidjs/router";
import { Component } from "solid-js";
import { GoogleAuthButton } from "../features/auth/GoogleAuthButton";
import { MicrosoftAuthButton } from "../features/auth/MicrosoftAuthButton";

interface Props {
    type: "Sign-in" | "Sign-up";
}

export const AuthPage: Component<Props> = ({ type }) => {
    return (
        <>
            <div class="w-[100vw] flex flex-row h-[100vh]">
                <div class="relative w-1/2 bg-white max-lg:hidden">
                    <div class="absolute top-1/3 right-1/2 translate-x-1/2 flex flex-col items-center gap-3">
                        <h1 class="text-5xl">Ekklesia</h1>
                        <p>
                            A modern bible study tool rooted in <span class="italic">community</span>.
                        </p>
                    </div>
                    <hr />
                </div>
                <div class="w-1/2 bg-black relative max-lg:w-full">
                    <div class="absolute w-72 top-1/3 right-1/2 translate-x-1/2 flex flex-col gap-6">
                        <h2 class="text-2xl text-white">Login</h2>
                        {/* TODO: support auth with email */}
                        <div class="flex flex-col gap-0.5 w-full">
                            <label for="email-login" class="text-white">
                                Email
                            </label>
                            <input id="email-login" type="email" />
                        </div>
                        <div class="w-full flex gap-1.5 items-center">
                            <hr class="w-1/2 border-t-[1px] border-white" />
                            <p class="text-white">or</p>
                            <hr class="w-1/2 border-t-[1px] border-white" />
                        </div>
                        <div class="flex items-center gap-3"></div>
                        <div class="flex gap-10 items-center">
                            <GoogleAuthButton type={type} />
                            <MicrosoftAuthButton type={type} />
                        </div>
                        <div class="text-white text-xs">
                            Don't have an account? <A href="/sign-up">Sign Up</A>
                        </div>
                    </div>
                </div>
            </div>
        </>
    );
};
