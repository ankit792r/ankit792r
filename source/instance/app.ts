import type { FastifyInstance } from "fastify";
import { join } from "node:path";
import type { AppPluginOptions } from "./server";
import autoload from "@fastify/autoload";
import { errorHandler, notFoundHandler } from "./errors/handlers";
import type { RedisClient } from "bun";
import type { ICache } from "../shared/cache/cache.interface";
import type { Collection, MongoClient } from "mongodb";
import type { IEmailService } from "../shared/email/email.interface";
import type { Category } from "../entities/category.model";
import type { BlogPost } from "../entities/post.model";
import type { Tag } from "../entities/tag.model";

export type DependencyOverrides = {
  mongoClient?: MongoClient;
  redisClient?: RedisClient;
  defaultCache?: ICache;

  blogPostCollection?: Collection<BlogPost>;
  tagCollection?: Collection<Tag>;
  categoryCollection?: Collection<Category>;

  emailService?: IEmailService;
};

export async function app(fastify: FastifyInstance, opts?: AppPluginOptions) {
  fastify.setErrorHandler(errorHandler);
  fastify.setNotFoundHandler(notFoundHandler);

  fastify.register(autoload, {
    dir: join(__dirname, "plugins"),
    options: opts,
  });

  fastify.log.info("plugging: ROUTES into app");
  fastify.register(autoload, {
    dir: join(__dirname, "routes"),
    prefix: opts?.appOptions?.apiPrefix,
  });
}
