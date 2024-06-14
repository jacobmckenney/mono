import { Capacitor } from "@capacitor/core";
import { z } from "zod";

const platformSchema = z.enum(["web", "ios"]).default("web");

export const usePlatform = () => {
    const platform = platformSchema.parse(Capacitor.getPlatform());
    const isWeb = platform === "web";
    const isIos = platform === "ios";
    return { platform, isWeb, isIos };
};
