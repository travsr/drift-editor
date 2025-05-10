import { z } from "zod";

const baseSchema = z.object({
    name: z.string(),
    path: z.string(),
    type: z.enum(["file", "folder"]),
    isExpanded: z.boolean().optional(),
});

export type D_FileTreeNode = z.infer<typeof baseSchema> & {
    children?: D_FileTreeNode[];
};

export const D_FileTreeNodeSchema: z.ZodType<D_FileTreeNode> =
    baseSchema.extend({
        children: z.lazy(() => D_FileTreeNodeSchema.array()).optional(),
    });
