import { z } from "zod";

import { D_TabSchema } from "./D_Tab";
import { D_DocumentSchema } from "./D_Document";
import { D_DocumentFileSchema } from "./D_DocumentFile";
import { D_FileTreeNodeSchema } from "./D_FileTreeNode";
import { D_InterfaceSchema } from "./D_Interface";

export const D_AppStateSchema = z.object({
    tabs: z.array(D_TabSchema),
    content: z.object({
        documents: z.array(D_DocumentSchema.or(D_DocumentFileSchema)),
    }),
    fileTree: D_FileTreeNodeSchema,
    ui: D_InterfaceSchema,
});
export type D_UIState = z.infer<typeof D_AppStateSchema>;
