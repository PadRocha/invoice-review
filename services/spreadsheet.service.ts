import {
  INVOICE_KEY_COLUMN,
  INVOICE_PRICE_COLUMN,
  readInvoiceRows,
  readSystemRows,
  readWorkbook,
  SYSTEM_KEY_COLUMN,
  SYSTEM_PRICE_COLUMN,
} from "@spreadsheet";
import type { InvoiceRow } from "@interfaces/invoice_row.interface.ts";
import type { SystemRow } from "@interfaces/system_row.interface.ts";

export async function loadInvoiceRows(path: string): Promise<InvoiceRow[]> {
  const workbook = await readWorkbook(path);
  return readInvoiceRows(workbook, path, {
    key_column: INVOICE_KEY_COLUMN,
    price_column: INVOICE_PRICE_COLUMN,
  });
}

export async function loadSystemRows(
  path: string,
  priority_index: number,
): Promise<SystemRow[]> {
  const workbook = await readWorkbook(path);
  return readSystemRows(workbook, path, {
    key_column: SYSTEM_KEY_COLUMN,
    price_column: SYSTEM_PRICE_COLUMN,
    priority_index,
  });
}
