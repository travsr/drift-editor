import type { z } from "zod";
import { D_DocumentSchema } from "./D_Document";

export const D_DocumentTerminalSchema = D_DocumentSchema.extend({});
export type D_DocumentTerminal = z.infer<typeof D_DocumentTerminalSchema>;
