import z from "zod";
import { BoardIdSchema } from "../../../entities/board.model";

export const BoardIdParamDtoSchema = z.object({
  id: BoardIdSchema,
});

export type BoardIdParamDto = z.infer<typeof BoardIdParamDtoSchema>;
