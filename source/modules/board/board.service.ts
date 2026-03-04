import type { Collection } from "mongodb";
import {
  BoardSchema,
  createBoardId,
  type Board,
  type BoardId,
} from "../../entities/board.model";
import type { UserService } from "../user/user.service";
import type { Towall } from "../../entities/towall.model";
import type { UserId } from "../../entities/user.model";
import type { BoardCreateDto } from "./dto/board-create.dto";
import {
  BoardFullResponseDtoSchema,
  type BoardFullResponseDto,
} from "./dto/board-full.dto";
import { AppError } from "../../instance/errors/app-error";
import type { ListResponseDto } from "../../shared/list-response";
import { TowallBasicResponseDtoSchema } from "../towall/dto/towall-basic.dto";
import type { UserBasicResponseDto } from "../user/dto/user-response.dto";
import { BoardError } from "./board.error";
import {
  BoardBasicResponseDtoSchema,
  type BoardBasicResponseDto,
} from "./dto/board-basic.dto";
import type { BoardNameCheckDto } from "./dto/board-name-check.dto";
import type { BoardSearchQueryDto } from "./dto/board-search-query.dto";
import type { BoardUpdateDto } from "./dto/board-update.dto";

export class BoardService {
  constructor(
    private readonly boardCollection: Collection<Board>,
    private readonly userService: UserService,
    private readonly towallCollection: Collection<Towall>,
  ) { }

  // TODO: Check if user has reached the maximum number of boards
  // TODO: And other business rules
  async createBoard(
    userId: UserId,
    dto: BoardCreateDto,
  ): Promise<BoardFullResponseDto> {
    const userBasicInfo = await this.userService.getUserBasicInfoById(userId);

    const newBoardObject = BoardSchema.parse({
      ...dto,
      _id: createBoardId(),
      userId,
      headerImages: [],
      meta: { totalCount: 0, totalSize: 0 },
      createdAt: new Date().toISOString(),
      updatedAt: new Date().toISOString(),
    });

    const newBoard = await this.boardCollection.insertOne(newBoardObject);
    if (!newBoard.acknowledged)
      throw new AppError(BoardError.BoardCreationFailed);

    const fullBoardInfo = BoardFullResponseDtoSchema.parse({
      ...newBoardObject,
      creator: userBasicInfo,
      towalls: [],
    });

    return fullBoardInfo;
  }

  async updateBoard(
    boardId: BoardId,
    dto: BoardUpdateDto,
  ): Promise<BoardFullResponseDto> {
    const board = await this.getBoardByIdOrThrow(boardId);
    const updatedBoard = await this.boardCollection.updateOne(
      { _id: boardId },
      { $set: { ...dto, updatedAt: new Date() } },
    );
    if (updatedBoard.matchedCount === 0)
      throw new AppError(BoardError.BoardNotFound);
    return this.getFullBoardById(board._id);
  }

  // TODO: Add pagination
  async listUserBoards(
    userId: UserId,
  ): Promise<ListResponseDto<BoardBasicResponseDto>> {
    const boards = await this.boardCollection // TODO: Add projection to only get the fields we need
      .find({ userId })
      .sort({ createdAt: -1 })
      .toArray();

    return {
      items: boards.map((board) => BoardBasicResponseDtoSchema.parse(board)),
      page: undefined,
    };
  }

  // TODO: Add pagination
  async searchBoards(
    dto: BoardSearchQueryDto,
  ): Promise<ListResponseDto<BoardBasicResponseDto>> {
    console.log(dto);

    const boards = await this.boardCollection
      .find({ name: { $regex: dto.query, $options: "i" } }) // TODO: Add search by tags and description
      .sort({ createdAt: -1 })
      .toArray();

    return {
      items: boards.map((board) => BoardBasicResponseDtoSchema.parse(board)),
      page: undefined,
    };
  }

  async boardNameCheck(dto: BoardNameCheckDto): Promise<void> {
    const isAvailable =
      (await this.boardCollection.countDocuments({ name: dto.name })) === 0;
    if (!isAvailable) throw new AppError(BoardError.BoardNameTaken);
  }

  // INFO: this method should only be called with desktop app
  // INFO: where we will have `username/board-name` format
  // INFO: return will be same as getFullBoard
  async getFullBoardByBoardNameAndUsername(
    boardName: string,
    username: string,
  ): Promise<BoardFullResponseDto> {
    const creator = await this.userService.getUserBasicInfoByUsername(username);
    const board = await this.boardCollection.findOne({
      name: boardName,
      userId: creator._id,
    });
    if (!board) throw new AppError(BoardError.BoardNotFound);

    return this.constructFullBoard(board, creator);
  }

  async getFullBoardById(boardId: BoardId): Promise<BoardFullResponseDto> {
    const board = await this.boardCollection.findOne({ _id: boardId });
    if (!board) throw new AppError(BoardError.BoardNotFound);
    const creator = await this.userService.getUserBasicInfoById(board.userId);

    return this.constructFullBoard(board, creator);
  }

  private async constructFullBoard(
    board: Board,
    user: UserBasicResponseDto,
  ): Promise<BoardFullResponseDto> {
    const query = { boardId: board._id, userId: user._id };
    const limit = 10;

    // Execute queries in parallel for better performance
    const [towalls, totalCount] = await Promise.all([
      this.towallCollection
        .find(query)
        .sort({ createdAt: -1 })
        .limit(limit)
        .toArray(),
      this.towallCollection.countDocuments(query),
    ]);

    const totalPages = Math.ceil(totalCount / limit);
    const pageToken =
      totalPages > 0
        ? JSON.stringify({
          page: 1,
          limit,
          totalPages,
          hasNextPage: totalPages > 1,
        })
        : undefined;

    // Parse towalls in a single pass
    const parsedTowalls = towalls.map((towall) =>
      TowallBasicResponseDtoSchema.parse({
        _id: towall._id,
        title: towall.title,
        fileSize: towall.fileSize,
        mimeType: towall.mimeType,
        createdAt: towall.createdAt,
        updatedAt: towall.updatedAt,
        defaultUrl: towall.fileUrl.default,
      }),
    );

    return BoardFullResponseDtoSchema.parse({
      ...board,
      creator: user,
      towalls: {
        items: parsedTowalls,
        page: pageToken,
      },
    });
  }

  async getBoardByIdOrThrow(boardId: BoardId): Promise<Board> {
    const board = await this.boardCollection.findOne({ _id: boardId });
    if (!board) throw new AppError(BoardError.BoardNotFound);
    return board;
  }
}
