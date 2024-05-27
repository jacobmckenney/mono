import { Component } from "solid-js";
import { GoogleAuthButton } from "./features/auth/GoogleAuthButton";
import { MicrosoftAuthButton } from "./features/auth/MicrosoftAuthButton";
import "./index.css";

export const App: Component = () => {
  return (
    <div class=" w-[100vw]">
      <GoogleAuthButton type="Sign-in" />
      <MicrosoftAuthButton type="Sign-in" />
    </div>
  );
};
