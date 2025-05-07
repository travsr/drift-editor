import { z } from "zod";
import { D_DocumentRefSchema } from "./D_DocumentRef";

export const D_TabSchema = z.object({
    title: z.string(),
    documentRefs: z.array(D_DocumentRefSchema),
});
export type D_Tab = z.infer<typeof D_TabSchema>;
