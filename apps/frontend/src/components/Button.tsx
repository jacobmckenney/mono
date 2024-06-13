import { Component, JSX, JSXElement, Resource } from "solid-js";

export const Button: Component<
    {
        href?: Resource<string>;
        children?: JSXElement;
    } & JSX.ButtonHTMLAttributes<HTMLButtonElement>
> = ({ href, children, ...rest }) => {
    return (
        <button class="text-white" {...rest}>
            {children}
        </button>
    );
};
