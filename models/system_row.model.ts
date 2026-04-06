import type { SystemRow } from "@interfaces/system_row.interface.ts";

export function buildSystemRow(
  row_number: number,
  key: string,
  price: number,
  source_file: string,
  source_sheet: string,
  priority_index: number,
): SystemRow {
  return {
    row_number,
    key,
    price,
    source_file,
    source_sheet,
    priority_index,
  };
}
