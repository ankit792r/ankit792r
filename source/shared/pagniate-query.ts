import z from "zod";

export const PaginatedQueryDtoSchema = z
  .object({
    page: z.string().optional().meta({ example: "1" }),
    limit: z.number().int().positive().optional().default(10),
  })
  .meta({ description: "Paginated query" });
