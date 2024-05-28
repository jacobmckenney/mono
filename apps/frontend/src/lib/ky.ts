import ky from "ky";
import { getBaseUrl } from "./url";

export const ekklesiaApi = ky.extend({
    prefixUrl: getBaseUrl().server,
    credentials: "include",
    hooks: {
        afterResponse: [
            (_input, _opts, res) => {
                // Redirect if the response redirect is to our trusted client
                if (res.redirected && res.url.startsWith(getBaseUrl().client)) {
                    window.location.href = res.url;
                    return res;
                }
                return res;
            },
        ],
    },
});
