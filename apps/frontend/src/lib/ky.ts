import ky from "ky";
import { getBaseUrl } from "./url";

export const ekklesiaApi = ky.extend({
    prefixUrl: getBaseUrl().server,
    credentials: "include",
});
