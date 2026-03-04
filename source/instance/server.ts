import Fastify from "fastify";
import { app, type DependencyOverrides } from "./app";

export interface AppOptions {
  dependencyOverrides: DependencyOverrides;
  apiPrefix: string;
}

export interface AppPluginOptions {
  appOptions?: AppOptions;
}

declare module "fastify" {
  interface FastifyInstance {
    appOptions: AppOptions;
  }
}

export const createServer = async () => {
  const server = Fastify({
    logger: {
      formatters: {
        level: (label: string) => ({
          label,
        }),
      },
    },
    pluginTimeout: 15000,
    disableRequestLogging: false,
    trustProxy: true,
    bodyLimit: 1048576,
  }).withTypeProvider();

  server.register(app, {
    apiPrefix: "/api",
    dependencyOverrides: {},
  } as AppOptions);

  return server;
};
