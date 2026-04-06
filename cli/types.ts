export interface HelpCliOptions {
  command: "help";
}

export interface ReviewCliOptions {
  invoice_path?: string;
  system_paths: string[];
  output_path?: string;
  json_path?: string;
  sensitivity?: number;
}

export interface ValidReviewCliOptions extends ReviewCliOptions {
  invoice_path: string;
}

export type CliOptions = HelpCliOptions | ReviewCliOptions;
