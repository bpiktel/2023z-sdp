import { z } from "zod";

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
