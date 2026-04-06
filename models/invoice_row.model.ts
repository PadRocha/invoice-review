import type { InvoiceRow } from "@interfaces/invoice_row.interface.ts";

export function buildInvoiceRow(
  row_number: number,
  key: string,
  price: number,
  source_file: string,
): InvoiceRow {
  return {
    row_number,
    key,
    price,
    source_file,
  };
}
