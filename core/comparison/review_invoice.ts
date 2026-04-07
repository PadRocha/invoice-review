import type { InvoiceRow } from "@interfaces/invoice_row.interface.ts";
import type { MultipleMatch } from "@interfaces/multiple_match.interface.ts";
import type { PriceMismatch } from "@interfaces/price_mismatch.interface.ts";
import type { ReviewReport } from "@interfaces/review_report.interface.ts";
import type { SystemRow } from "@interfaces/system_row.interface.ts";
import { buildReviewReport } from "@models";
import { calculatePercentageVariation } from "@utils/percentage.ts";
import { calculateDiscountedPrice } from "./discounts.ts";
import type { ReviewInvoiceConfig } from "./types.ts";
import { resolveSystemMatch } from "./resolve_match.ts";

const PERCENTAGE_FORMULA = "precio_usado / precio_sistema * 100 - 100";

export function reviewInvoice(
  invoice_file: string,
  invoice_rows: InvoiceRow[],
  system_files: string[],
  system_rows: SystemRow[],
  config: ReviewInvoiceConfig = {},
): ReviewReport {
  const system_rows_by_file = groupRowsByFile(system_rows);
  const discounts = config.discounts ?? [];
  const price_mismatches: PriceMismatch[] = [];
  const missing_keys: string[] = [];
  const multiple_matches: MultipleMatch[] = [];

  for (const invoice_row of invoice_rows) {
    const resolution = resolveSystemMatch(
      invoice_row.key,
      system_rows_by_file,
      system_files,
    );

    switch (resolution.kind) {
      case "not_found": {
        missing_keys.push(invoice_row.key);
        break;
      }
      case "multiple": {
        multiple_matches.push({
          key: invoice_row.key,
          invoice_row_number: invoice_row.row_number,
          system_files: [
            ...new Set(resolution.rows.map((row) => row.source_file)),
          ],
          system_rows: resolution.rows.map((row) => row.row_number),
        });
        break;
      }
      case "found": {
        const discounted_price = calculateDiscountedPrice(
          invoice_row.price,
          discounts,
        );

        if (discounted_price.adjusted_price === resolution.row.price) {
          break;
        }

        const percentage_result = calculatePercentageVariation(
          discounted_price.adjusted_price,
          resolution.row.price,
        );

        if (isHiddenBySensitivity(percentage_result, config.sensitivity)) {
          break;
        }

        price_mismatches.push({
          key: invoice_row.key,
          invoice_price: discounted_price.original_price,
          compared_invoice_price: discounted_price.adjusted_price,
          system_price: resolution.row.price,
          system_file: resolution.row.source_file,
          system_sheet: resolution.row.source_sheet,
          invoice_row_number: invoice_row.row_number,
          system_row_number: resolution.row.row_number,
          percentage_formula: PERCENTAGE_FORMULA,
          percentage_result,
        });
        break;
      }
    }
  }

  return buildReviewReport(
    invoice_file,
    system_files,
    discounts,
    invoice_rows.length,
    price_mismatches,
    missing_keys,
    multiple_matches,
  );
}

function groupRowsByFile(system_rows: SystemRow[]): Map<string, SystemRow[]> {
  const grouped_rows = new Map<string, SystemRow[]>();

  for (const system_row of system_rows) {
    const file_rows = grouped_rows.get(system_row.source_file) ?? [];
    file_rows.push(system_row);
    grouped_rows.set(system_row.source_file, file_rows);
  }

  return grouped_rows;
}

function isHiddenBySensitivity(
  percentage_result: number,
  sensitivity: number | undefined,
): boolean {
  if (sensitivity === undefined) {
    return false;
  }

  return percentage_result <= 0 && percentage_result > sensitivity;
}
