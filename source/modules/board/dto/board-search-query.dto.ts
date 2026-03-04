import z from "zod";

export const BoardSearchQueryDtoSchema = z.object({
  query: z.string().min(1).max(100),
});

export type BoardSearchQueryDto = z.infer<typeof BoardSearchQueryDtoSchema>;
