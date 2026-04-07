import type { MultipleMatch } from "./multiple_match.interface.ts";
import type { PriceMismatch } from "./price_mismatch.interface.ts";

export interface ReviewSummary {
  total_rows_reviewed: number;
  total_price_mismatches: number;
  total_missing_keys: number;
  total_multiple_matches: number;
}

export interface ReviewReport {
  invoice_file: string;
  system_files: string[];
  discounts: number[];
  price_mismatches: PriceMismatch[];
  missing_keys: string[];
  multiple_matches: MultipleMatch[];
  summary: ReviewSummary;
}
