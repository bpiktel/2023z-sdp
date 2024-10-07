import { z } from "zod";
import { sampleResultListSchema } from "./sampleSchemas.ts";

export const idSchema = z.string();

export type Id = z.infer<typeof idSchema>;

export const experimentSchema = z.object({
  id: idSchema,
  name: z.string(),
  sample_ids: z.array(z.string()),
  is_public: z.optional(z.boolean()),
});

export type Experiment = z.infer<typeof experimentSchema>;

export const experimentListSchema = z.array(experimentSchema);

export type ExperimentList = z.infer<typeof experimentListSchema>;

export const experimentResultSchema = z.object({
  id: idSchema,
  sample_results: sampleResultListSchema,
  training: z.boolean(),
  user: z.string(),
});

export type ExperimentResult = z.infer<typeof experimentResultSchema>;

export const experimentResultListSchema = z.array(experimentResultSchema);

export type ExperimentResultList = z.infer<typeof experimentResultListSchema>;
