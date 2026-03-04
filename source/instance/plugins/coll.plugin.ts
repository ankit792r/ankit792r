import { type FastifyInstance } from "fastify";
import fp from "fastify-plugin";
import type { DependencyOverrides } from "../app";
import { createCollection } from "../../shared/mongodb/colls-factory";
import {
  BlogPostCollectionConfig,
  type BlogPost,
} from "../../entities/post.model";
import { TagCollectionConfig, type Tag } from "../../entities/tag.model";
import {
  CategoryCollectionConfig,
  type Category,
} from "../../entities/category.model";
import type { Collection } from "mongodb";

export default fp(
  async (fastify: FastifyInstance, overrides: DependencyOverrides) => {
    fastify.log.info("plugging: REPO into app");

    const blogPostCollection =
      overrides.blogPostCollection ??
      (await createCollection<BlogPost>(
        BlogPostCollectionConfig,
        fastify.mongoClient,
      ));

    const tagCollection =
      overrides.tagCollection ??
      (await createCollection<Tag>(TagCollectionConfig, fastify.mongoClient));

    const categoryCollection =
      overrides.categoryCollection ??
      (await createCollection<Category>(
        CategoryCollectionConfig,
        fastify.mongoClient,
      ));

    fastify.decorate("blogPostCollection", blogPostCollection);
    fastify.decorate("tagCollection", tagCollection);
    fastify.decorate("categoryCollection", categoryCollection);
  },
  { name: "collection", dependencies: ["db"] },
);

declare module "fastify" {
  interface FastifyInstance {
    blogPostCollection: Collection<BlogPost>;
    tagCollection: Collection<Tag>;
    categoryCollection: Collection<Category>;
  }
}
