import { z } from "zod";

export const D_DocumentTypeSchema = z.enum(["file", "terminal", "settings"]);
export type D_DocumentType = z.infer<typeof D_DocumentTypeSchema>;

export const D_DocumentSchema = z.object({
    id: z.string(),
    title: z.string(),
    type: D_DocumentTypeSchema,
});
export type D_Document = z.infer<typeof D_DocumentSchema>;
