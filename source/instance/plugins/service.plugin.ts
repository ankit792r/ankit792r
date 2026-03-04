import { type FastifyInstance } from "fastify";
import fp from "fastify-plugin";
import type { DependencyOverrides } from "../app";
import { BoardService } from "../../modules/board/board.service";
import { createEmailService } from "../../shared/email/email-factory";

export default fp(
  async (fastify: FastifyInstance, overrides: DependencyOverrides) => {
    fastify.log.info("plugging: SERVICE in app");

    const emailService = overrides.emailService ?? createEmailService();

    fastify.decorate("emailService", emailService);
  },
  { name: "service", dependencies: ["collection", "cache", "storage"] },
);

declare module "fastify" {
  interface FastifyInstance {
    boardService: BoardService;
  }
}
