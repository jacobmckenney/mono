import * as Kobalte from "@kobalte/core/button";
import { VariantProps, cva } from "class-variance-authority";
import { Component, JSX, JSXElement, children as solidChildren } from "solid-js";

const buttonVariants = cva(
    [
        "border",
        "rounded-md",
        "shadow-sm",
        "stack",
        "font-semibold",
        "transition-all",
        "disabled:cursor-not-allowed",
        "focus:outline-none",
        "focus:ring-2",
        "ring-gray-9/50",
        "hover:brightness-95",
        "disabled:brightness-90",
        "active:scale-95",
        "disabled:active:scale-100",
    ],
    {
        variants: {
            theme: {
                // primary: [],
                // secondary: [],
                transparent: ["p-1", "bg-transparent", "border-none", "focus:ring-0", "shadow-none"],
                white: ["bg-white", "text-black", "border-gray-3"],
                black: ["bg-black", "text-white", "border-gray-3"],
                normal: ["bg-black", "text-white", "border-gray-3", "dark:bg-white", "dark:text-black"],
                inverted: ["bg-white", "text-black", "border-gray-3", "dark:bg-black", "dark:text-white"],
            },
            size: {
                xs: ["text-xs", "py-0.5", "px-2"],
                sm: ["text-sm", "py-1", "px-3"],
                md: ["text-base", "py-2", "px-4"],
                none: ["p-0"],
            },
        },
        defaultVariants: {
            theme: "normal",
            size: "sm",
        },
    }
);

export interface ButtonProps
    extends VariantProps<typeof buttonVariants>,
        Omit<JSX.HTMLAttributes<HTMLButtonElement>, "ref">,
        Kobalte.ButtonRootProps {
    children?: JSXElement;
}

export const Button: Component<ButtonProps> = ({ children, size, theme, ...rest }) => {
    const c = solidChildren(() => children);

    return (
        <Kobalte.Button {...rest} class={buttonVariants({ size, theme, class: rest.class })}>
            {c()}
        </Kobalte.Button>
    );
};
