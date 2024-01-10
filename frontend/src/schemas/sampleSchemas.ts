import { z } from "zod";

export const idSchema = z.object({
  tb: z.string(),
  id: z.object({
    String: z.string()
  })
});

export type Id = z.infer<typeof idSchema>;

export const sampleSchema = z.object({
  id: idSchema,
  name: z.string(),
  azimuth: z.number(),
  elevation: z.number()
});

export type Sample = z.infer<typeof sampleSchema>;

export const sampleListSchema = z.array(sampleSchema);

export type SampleList = z.infer<typeof sampleListSchema>;

export const sampleResultSchema = z.object({
  sample_id: z.string(),
  azimuth: z.number(),
  elevation: z.number()
});

export type SampleResult = z.infer<typeof sampleResultSchema>;

export const sampleResultListSchema = z.array(sampleResultSchema);

export type SampleResultList = z.infer<typeof sampleResultListSchema>;