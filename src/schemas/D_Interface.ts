import { z } from "zod";

export const D_InterfaceSchema = z.object({
    isOverlayActive: z.boolean(),
    sidebar: z.enum(["tree", "tabs"]),
});
export type D_Interface = z.infer<typeof D_InterfaceSchema>;
