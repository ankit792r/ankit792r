import z from "zod";
import { createIdFactoryFromIdSchema, idSchema } from "./id-factory";
import type { CollectionConfig } from "../shared/mongodb/colls-factory";

export const CategoryIdSchema = idSchema({
  brand: "CategoryId",
  prefix: "ct_",
});
export type CategoryId = z.infer<typeof CategoryIdSchema>;
export const createCategoryId = createIdFactoryFromIdSchema(CategoryIdSchema);

export const CategorySchema = z.object({
  _id: CategoryIdSchema,
  name: z.string().max(60),
  slug: z.string().max(100),
  description: z.string().optional(),
  parentId: CategoryIdSchema.optional(),
});

export type Category = z.infer<typeof CategorySchema>;

export const CategoryCollectionConfig: CollectionConfig<Category> = {
  indices: [],
  name: "categories",
  primaryKey: "_id",
  schema: CategorySchema,
  schemaVersion: 1,
};
