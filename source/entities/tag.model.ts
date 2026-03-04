import z from "zod";
import { createIdFactoryFromIdSchema, idSchema } from "./id-factory";
import type { CollectionConfig } from "../shared/mongodb/colls-factory";

export const TagIdSchema = idSchema({
  brand: "TagId",
  prefix: "ct_",
});
export type TagId = z.infer<typeof TagIdSchema>;
export const createTagId = createIdFactoryFromIdSchema(TagIdSchema);

export const TagSchema = z.object({
  _id: TagIdSchema,
  name: z.string().min(2).max(100),
  slug: z.string().min(2).max(150),
  createdAt: z.iso.date(),
});

export type Tag = z.infer<typeof TagSchema>;

export const TagCollectionConfig: CollectionConfig<Tag> = {
  indices: [],
  name: "tags",
  primaryKey: "_id",
  schema: TagSchema,
  schemaVersion: 1,
};
