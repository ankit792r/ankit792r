import fastifyStatic from "@fastify/static";
import type { FastifyInstance } from "fastify";
import fastifyPlugin from "fastify-plugin";
import * as path from "path";

export default fastifyPlugin(
  async (fastify: FastifyInstance) => {
    // Resolve public directory relative to the backend app directory
    // Works in both development (src) and production (dist) contexts
    const publicPath = path.resolve(__dirname, "../../uploads");

    await fastify.register(fastifyStatic, {
      root: publicPath,
      prefix: "/uploads/",
      decorateReply: true,
    });
  },
  { name: "static" },
);
