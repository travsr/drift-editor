import { z } from "zod";
import { D_DocumentSchema } from "./D_Document";

export const D_DocumentFileSchema = D_DocumentSchema.extend({
    filePath: z.string(),
    buffer: z.string(),
    status: z.enum(["new", "saved-to-fs", "modified"]),
});
export type D_DocumentFile = z.infer<typeof D_DocumentFileSchema>;
