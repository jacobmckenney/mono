import { match } from "ts-pattern";
export const getBaseUrl = () => {
    const client = window.location.origin;
    // TODO: fill this once I have a production URL
    const server = match(client)
        .with("app.ekklesia.dev", () => "https://api.ekklesia.dev")
        .otherwise(() => "http://localhost:8080");
    return { client, server };
};
