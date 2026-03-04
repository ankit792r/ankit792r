export interface IEmail {
  subject: string;
  receiver: string[];
  contentType: "HTML" | "Text";
  content: string;
  attachments?: {
    data: Buffer;
    fileName: string;
  }[];
}

export interface IEmailService {
  sendEmail(mailData: IEmail): Promise<void>;
}
