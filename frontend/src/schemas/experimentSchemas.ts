import { z } from "zod";
import {sampleResultListSchema} from "./sampleSchemas.ts";

export const idSchema = z.object({
  tb: z.string(),
  id: z.object({
    String: z.string()
  })
});

export type Id = z.infer<typeof idSchema>;

export const experimentSchema = z.object({
  id: idSchema,
  name: z.string(),
  sample_ids: z.array(z.string())
});

export type Experiment = z.infer<typeof experimentSchema>;

export const experimentListSchema = z.array(experimentSchema);

export type ExperimentList = z.infer<typeof experimentListSchema>;



export const experimentResultSchema = z.object({
  id: idSchema,
  sample_results: z.array(sampleResultListSchema)
});

export type ExperimentResult = z.infer<typeof experimentResultSchema>;

export const experimentResultsSchema = z.array(experimentResultSchema);

export type ExperimentResults = z.infer<typeof experimentResultsSchema>;