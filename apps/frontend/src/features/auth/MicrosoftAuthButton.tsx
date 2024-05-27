import { createMutation } from "@tanstack/solid-query";
import { Component } from "solid-js";
import { Button } from "../../components/Button";
import { authUrlSchema } from "./GoogleAuthButton";

interface Props {
  type: "Sign-in" | "Sign-up";
}

export const MicrosoftAuthButton: Component<Props> = ({ type }) => {
  const getLink = createMutation(() => ({
    mutationFn: async () => {
      const res = await fetch("http://localhost:8080/auth/link/microsoft", {
        headers: {
          "Content-Type": "application/json",
        },
      });
      const json = await res.json();
      return authUrlSchema.parse(json).url;
    },
    mutationKey: ["get-google-auth-url"],
    onSuccess: (url) => {
      window.location.href = url;
    },
  }));
  return (
    <>
      <Button onClick={() => getLink.mutate()}>{type} with Microsoft</Button>
    </>
  );
};
