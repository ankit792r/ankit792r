import env from "../../instance/env";
import type { IEmailService } from "./email.interface";
import { MockEmailService } from "./mock.email";
import { PrintToConsoleEmailService } from "./print.email";

export function createEmailService(): IEmailService {
  switch (env.EMAIL_SERVICE_IMPL) {
    case "mock":
      return new MockEmailService();
    case "print":
      return new PrintToConsoleEmailService();
    default:
      throw new Error(
        `Invalid EMAIL_SERVICE_IMPL found: ${env.EMAIL_SERVICE_IMPL}`,
      );
  }
}
