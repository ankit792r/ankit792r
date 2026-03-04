import z from "zod";
import { BoardSchema } from "../../../entities/board.model";
import { UserBasicResponseDtoSchema } from "../../user/dto/user-response.dto";

export const BoardFullResponseDtoSchema = BoardSchema.extend({
  towalls: z.object({
    items: z.string(), // z.array(TowallBasicResponseDtoSchema),
    page: z.string().optional(),
  }),
  creator: UserBasicResponseDtoSchema,
});

export type BoardFullResponseDto = z.infer<typeof BoardFullResponseDtoSchema>;
