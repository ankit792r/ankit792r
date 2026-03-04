import z from "zod";
import { BoardSchema } from "../../../entities/board.model";

export const BoardBasicResponseDtoSchema = BoardSchema.pick({
    _id: true,
    name: true, 
    headerImages: true,
    isPrivate: true,
    tags: true,
    createdAt: true,
    updatedAt: true,
});

export type BoardBasicResponseDto = z.infer<typeof BoardBasicResponseDtoSchema>;
