import { z } from "zod";
import { D_DocumentTypeSchema } from "./D_Document";

export const D_DocumentRefSchema = z.object({
    id: z.string(),
    title: z.string(),
    type: D_DocumentTypeSchema,
});
export type D_DocumentRef = z.infer<typeof D_DocumentRefSchema>;
