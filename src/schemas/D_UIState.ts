import { z } from "zod";

import { D_TabSchema } from "./D_Tab";
import { D_DocumentSchema } from "./D_Document";
import { D_DocumentFileSchema } from "./D_DocumentFile";

export const D_TabViewSchema = z.object({
    tabs: z.array(D_TabSchema),
});
export type D_TabViewProps = z.infer<typeof D_TabViewSchema>;

export const D_ContentViewSchema = z.object({
    documents: z.array(D_DocumentSchema.or(D_DocumentFileSchema)),
});
export type D_ContentViewProps = z.infer<typeof D_ContentViewSchema>;

export const D_DetailViewSchema = z.object({
    type: z.enum(["fileInfo", "explorer", "diagnostics", "git", "custom"]),
});
export type D_DetailViewProps = z.infer<typeof D_DetailViewSchema>;

export const D_UIStateSchema = z.object({
    tabView: D_TabViewSchema,
    contentView: D_ContentViewSchema,
    detailView: D_DetailViewSchema,
});
export type D_UIState = z.infer<typeof D_UIStateSchema>;
