import type { MultipleMatch } from "@interfaces/multiple_match.interface.ts";
import type { PriceMismatch } from "@interfaces/price_mismatch.interface.ts";
import type { ReviewReport } from "@interfaces/review_report.interface.ts";

export function buildReviewReport(
  invoice_file: string,
  system_files: string[],
  discounts: number[],
  total_rows_reviewed: number,
  price_mismatches: PriceMismatch[],
  missing_keys: string[],
  multiple_matches: MultipleMatch[],
): ReviewReport {
  return {
    invoice_file,
    system_files,
    discounts,
    price_mismatches,
    missing_keys,
    multiple_matches,
    summary: {
      total_rows_reviewed,
      total_price_mismatches: price_mismatches.length,
      total_missing_keys: missing_keys.length,
      total_multiple_matches: multiple_matches.length,
    },
  };
}
