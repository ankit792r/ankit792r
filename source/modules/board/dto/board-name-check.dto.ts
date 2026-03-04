import z from "zod";
import { BoardSchema } from "../../../entities/board.model";

export const BoardNameCheckDtoSchema = z.object({
  name: BoardSchema.shape.name,
});

export type BoardNameCheckDto = z.infer<typeof BoardNameCheckDtoSchema>;
