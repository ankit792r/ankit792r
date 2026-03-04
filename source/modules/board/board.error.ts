import type { AppErrorParams } from "../../instance/errors/app-error";

export const BoardError = {
  BoardNameTaken: {
    code: "board_name_taken",
    message: "Board name already taken",
    statusCode: 400,
  },
  BoardNotFound: {
    code: "board_not_found",
    message: "Board not found",
    statusCode: 404,
  },
  BoardCreationFailed: {
    code: "board_creation_failed",
    message: "Board creation failed",
    statusCode: 500,
  },
} as const satisfies Record<string, AppErrorParams>;

export type BoardErrorCode =
  (typeof BoardError)[keyof typeof BoardError]["code"];

export type BoardError = (typeof BoardError)[keyof typeof BoardError];
