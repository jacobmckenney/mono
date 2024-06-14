import ky from "ky";
import { getBaseUrl } from "./url";

export const ekklesiaApi = ky.extend({
    prefixUrl: getBaseUrl().server,
    credentials: "include",
    headers: {
        "Content-Type": "application/json",
    },
    // redirect: "follow",
});
