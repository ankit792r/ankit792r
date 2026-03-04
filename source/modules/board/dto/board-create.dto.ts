import z from "zod";

export const BoardCreateDtoSchema = z.object({
    name: z.string().min(1).max(100),
    description: z.string().max(1000).optional(),
    isPrivate: z.boolean().default(false),
    tags: z.array(z.string().min(1).max(50)).max(20).default([]),
});

export type BoardCreateDto = z.infer<typeof BoardCreateDtoSchema>;