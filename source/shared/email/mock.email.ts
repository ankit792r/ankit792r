import type { IEmail, IEmailService } from "./email.interface";

export class MockEmailService implements IEmailService {
  private sentEmails: IEmail[] = [];

  async sendEmail(mailData: IEmail): Promise<void> {
    this.sentEmails.push(mailData);
  }

  getSentEmails(): IEmail[] {
    return this.sentEmails;
  }

  clear(): void {
    this.sentEmails = [];
  }
}
