import z from "zod";
import { createIdFactoryFromIdSchema, idSchema } from "./id-factory";
import type { CollectionConfig } from "../shared/mongodb/colls-factory";

export const BlogPostIdSchema = idSchema({
  brand: "BlogPostId",
  prefix: "bp_",
});
export type BlogPostId = z.infer<typeof BlogPostIdSchema>;
export const createBlogPostId = createIdFactoryFromIdSchema(BlogPostIdSchema);

export const postSEOSchema = z.object({
  metaTitle: z.string().max(255).optional(),
  metaDescription: z.string().optional(),
  metaKeywords: z.array(z.string()).optional(),
  canonicalUrl: z.url().optional(),
  ogImageId: z.string().optional(),
});

export const BlogPostSchema = z.object({
  _id: BlogPostIdSchema,

  title: z.string().min(5).max(255),
  slug: z.string().min(3).max(255),
  excerpt: z.string().optional(),
  content: z.string().min(50),

  thumbnail: z.url().optional(),
  status: z.enum(["draft", "published", "archived"]).default("draft"),

  readingTime: z.number().int().positive().optional(),
  viewCount: z.number().int().positive().default(0),
  isFeatured: z.boolean().default(false),

  seo: postSEOSchema.optional(),

  publishedAt: z.iso.datetime(),
  createdAt: z.iso.datetime(),
  updatedAt: z.iso.datetime(),
});

export type BlogPost = z.infer<typeof BlogPostSchema>;

export const BlogPostCollectionConfig: CollectionConfig<BlogPost> = {
  indices: [],
  name: "posts",
  primaryKey: "_id",
  schema: BlogPostSchema,
  schemaVersion: 1,
};
