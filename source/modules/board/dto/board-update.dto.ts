import z from "zod";

export const BoardUpdateDtoSchema = z.object({
  name: z.string().min(1).max(100).optional(),
  description: z.string().max(1000).optional(),
  isPrivate: z.boolean().optional(),
  tags: z.array(z.string().min(1).max(50)).max(20).default([]).optional(),
});

export type BoardUpdateDto = z.infer<typeof BoardUpdateDtoSchema>;
