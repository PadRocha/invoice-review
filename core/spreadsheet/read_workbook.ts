import * as XLSX from "xlsx";
import { basename } from "@std/path";
import { SpreadsheetError, ValidationError } from "@errors";
import type { InvoiceRow } from "@interfaces/invoice_row.interface.ts";
import type { SystemRow } from "@interfaces/system_row.interface.ts";
import { buildInvoiceRow, buildSystemRow } from "@models";
import { parseNumber } from "@utils/number.ts";
import type { SheetReadOptions } from "./types.ts";

interface ParsedSheetRow {
  row_number: number;
  key: string;
  price: number;
  sheet_name: string;
}

interface ParsedSheetRowsResult {
  candidate_row_count: number;
  rows: ParsedSheetRow[];
}

type DenseSheetRow = Array<XLSX.CellObject | undefined>;
type DenseSheet = Array<DenseSheetRow | undefined>;

export async function readWorkbook(path: string): Promise<XLSX.WorkBook> {
  try {
    const file_data = await Deno.readFile(path);
    return XLSX.read(file_data, {
      type: "array",
      dense: true,
      raw: true,
      cellNF: false,
      cellText: true,
    });
  } catch (error) {
    if (error instanceof Deno.errors.NotFound) {
      throw new SpreadsheetError(`No existe el archivo: ${path}`);
    }

    throw new SpreadsheetError(
      `No se pudo leer el archivo de hoja de cálculo: ${path}. ${
        String(error)
      }`,
    );
  }
}

export function readInvoiceRows(
  workbook: XLSX.WorkBook,
  source_file: string,
  options: SheetReadOptions,
): InvoiceRow[] {
  const rows = readRowsFromWorkbook(workbook, source_file, options);

  return rows.map((entry) =>
    buildInvoiceRow(
      entry.row_number,
      entry.key,
      entry.price,
      source_file,
    )
  );
}

export function readSystemRows(
  workbook: XLSX.WorkBook,
  source_file: string,
  options: SheetReadOptions,
): SystemRow[] {
  const rows = readRowsFromWorkbook(workbook, source_file, options);

  return rows.map((entry) =>
    buildSystemRow(
      entry.row_number,
      entry.key,
      entry.price,
      source_file,
      entry.sheet_name,
      options.priority_index ?? 0,
    )
  );
}

function readRowsFromWorkbook(
  workbook: XLSX.WorkBook,
  source_file: string,
  options: SheetReadOptions,
): ParsedSheetRow[] {
  const rows: ParsedSheetRow[] = [];
  let candidate_row_count = 0;

  for (const { sheet_name, sheet } of getUsableSheets(workbook, source_file)) {
    const sheet_result = readRowsFromSheet(
      sheet,
      sheet_name,
      options,
    );

    candidate_row_count += sheet_result.candidate_row_count;
    rows.push(...sheet_result.rows);
  }

  if (rows.length === 0 && candidate_row_count > 0) {
    throw new ValidationError(
      `No se encontraron filas utilizables en ${
        basename(source_file)
      } usando clave en la columna ${options.key_column} y precio en la columna ${options.price_column}.`,
    );
  }

  return rows;
}

function getUsableSheets(
  workbook: XLSX.WorkBook,
  source_file: string,
): Array<{ sheet_name: string; sheet: XLSX.WorkSheet }> {
  if (workbook.SheetNames.length === 0) {
    throw new SpreadsheetError(
      `El archivo ${source_file} no contiene hojas utilizables.`,
    );
  }

  const sheets = workbook.SheetNames.flatMap((sheet_name) => {
    const sheet = workbook.Sheets[sheet_name];

    if (!sheet || !sheet["!ref"]) {
      return [];
    }

    return [{ sheet_name, sheet }];
  });

  if (sheets.length === 0) {
    throw new SpreadsheetError(
      `El archivo ${source_file} no contiene hojas utilizables.`,
    );
  }

  return sheets;
}

function readRowsFromSheet(
  sheet: XLSX.WorkSheet,
  sheet_name: string,
  options: SheetReadOptions,
): ParsedSheetRowsResult {
  const reference = sheet["!ref"];

  if (!reference) {
    return {
      candidate_row_count: 0,
      rows: [],
    };
  }

  const range = XLSX.utils.decode_range(reference);
  const key_column_index = XLSX.utils.decode_col(options.key_column);
  const price_column_index = XLSX.utils.decode_col(options.price_column);
  const rows: ParsedSheetRow[] = [];
  let candidate_row_count = 0;

  for (let row_index = range.s.r; row_index <= range.e.r; row_index += 1) {
    const spreadsheet_row_number = row_index + 1;
    const raw_key = readCellText(sheet, row_index, key_column_index);
    const raw_price = readCellValue(sheet, row_index, price_column_index);

    const key = normalizeKey(raw_key);

    if (!key) {
      continue;
    }

    candidate_row_count += 1;

    const price = parseNumber(raw_price);

    if (price === null) {
      continue;
    }

    rows.push({
      row_number: spreadsheet_row_number,
      key,
      price,
      sheet_name,
    });
  }

  return {
    candidate_row_count,
    rows,
  };
}

function readCellText(
  sheet: XLSX.WorkSheet,
  row_index: number,
  column_index: number,
): unknown {
  const cell = readCellObject(sheet, row_index, column_index);
  return cell?.w ?? cell?.v ?? null;
}

function readCellValue(
  sheet: XLSX.WorkSheet,
  row_index: number,
  column_index: number,
): unknown {
  const cell = readCellObject(sheet, row_index, column_index);
  return cell?.v ?? cell?.w ?? null;
}

function readCellObject(
  sheet: XLSX.WorkSheet,
  row_index: number,
  column_index: number,
): XLSX.CellObject | null {
  if (Array.isArray(sheet)) {
    const dense_sheet = sheet as unknown as DenseSheet;
    return dense_sheet[row_index]?.[column_index] ?? null;
  }

  const cell_address = XLSX.utils.encode_cell({
    r: row_index,
    c: column_index,
  });

  return sheet[cell_address] as XLSX.CellObject | undefined ?? null;
}

function normalizeKey(value: unknown): string {
  if (value === null || value === undefined) {
    return "";
  }

  return String(value).trim();
}
