import { A, useNavigate } from "@solidjs/router";
import { Component } from "solid-js";
import { Button } from "../../components/Button";
import { createAuthenticationPageGate } from "../../lib/hooks/auth";
import { ekklesiaApi } from "../../lib/ky";
import { GoogleAuthButton } from "./GoogleAuthButton";
import { MicrosoftAuthButton } from "./MicrosoftAuthButton";

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
    const navigate = useNavigate();
    const info = pageInfo[type];
    return (
        <>
            <div class="w-[100vw] flex flex-row h-[100vh]">
                <div class="relative w-1/2 bg-white max-xl:hidden">
                    <div class="absolute top-1/3 right-1/2 translate-x-1/2 flex flex-col items-center gap-3">
                        <h1 class="text-5xl">Ekklesia</h1>
                        <p class="text-center">
                            A modern bible study tool rooted in <span class="italic">community</span>.
                        </p>
                    </div>
                    <hr />
                </div>
                <div class="w-1/2 bg-black relative max-xl:w-full">
                    <h1 class="absolute top-10 left-10 text-4xl text-white xl:hidden">Ekklesia</h1>
                    <div class="absolute w-72 top-1/3 right-1/2 translate-x-1/2 flex flex-col gap-6">
                        <h2 class="text-2xl text-white">{info.title}</h2>
                        {/* TODO: support auth with email */}
                        <form
                            onSubmit={async (data) => {
                                data.preventDefault();
                                const formData = new FormData(data.currentTarget);
                                const val = formData.get(`email-${type}`) as string;
                                await ekklesiaApi.post(`auth/email`, {
                                    body: JSON.stringify({
                                        email: val,
                                    }),
                                });
                                navigate("/app");
                            }}
                            class="flex flex-col gap-0.5 w-full"
                        >
                            <label for={`email-${type}`} class="text-white">
                                Email
                            </label>
                            <input id={`email-${type}`} name={`email-${type}`} type="email" />
                            <Button type="submit" theme="white">
                                Login
                            </Button>
                        </form>
                        <div class="w-full flex gap-1.5 items-center justify-center">
                            <hr class="w-1/2 border-[0.5px] border-white" />
                            <p class="text-white">or</p>
                            <hr class="w-1/2 border-[0.5px] border-white" />
                        </div>
                        <div class="flex items-center gap-3"></div>
                        <div class="flex gap-10 items-center">
                            <GoogleAuthButton size="xs" theme="white" type={type} />
                            <MicrosoftAuthButton size="xs" theme="white" type={type} />
                        </div>
                        <div class="text-white text-xs">
                            Don't have an account?{" "}
                            <A class="hover:text-gray-3" href={`/auth/${info.other.href}`}>
                                {info.other.title}
                            </A>
                        </div>
                    </div>
                    <p class="absolute max-sm:right-1/2 max-sm:translate-x-1/2 max-sm:min-w-max bottom-10 right-10 text-white xl:hidden">
                        A modern bible study tool rooted in <span class="italic">community</span>.
                    </p>
                </div>
            </div>
        </>
    );
};
