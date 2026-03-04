import { decode } from "html-entities";
import type { IEmail, IEmailService } from "./email.interface";

export class PrintToConsoleEmailService implements IEmailService {
  async sendEmail(mailData: IEmail): Promise<void> {
    console.log("Email Sent");
    console.log("----------------------------------------");
    console.log(`To: ${mailData.receiver.join(", ")}`);
    console.log(`Subject: ${mailData.subject}`);
    console.log(`Content Type: ${mailData.contentType}`);
    console.log("----------------------------------------");
    console.log(decode(mailData.content));
    console.log("----------------------------------------");
  }
}
