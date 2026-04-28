import z from 'zod';
import { strategySchema } from '@/services/api/mapping_table';

/** Editor tab state */
export const FileTabSchema = z.object({
  id: z.string(),

  /** DAR mod root(To auto generate mapping table)  */
  inputPath: z.string(),
  outputPath: z.string(),

  /** mapping table raw string */
  text: z.string(),
  originalText: z.string().readonly(),

  /** struct from rust backend */
  dirty: z.boolean().optional(),
  cursorPos: z
    .object({
      lineNumber: z.number(),
      column: z.number(),
    })
    .optional(),

  tableGen: z.object({
    strategy: strategySchema,
  }),
});

export type FileTab = z.infer<typeof FileTabSchema>;
