import { assertEquals, assertRejects } from "@std/assert";
import * as XLSX from "xlsx";
import { reviewInvoice } from "@comparison";
import { ValidationError } from "@errors";
import {
  loadInvoiceRows,
  loadSystemRows,
} from "@services/spreadsheet.service.ts";

Deno.test("carga .xls y .xlsx reales de forma robusta con hojas, encabezados y filas vacias", async () => {
  const invoice_path = await writeWorkbookFile("xls", [
    {
      name: "Portada",
      rows: [
        ["Reporte de factura"],
        [null, null, "Clave", null, "Precio"],
      ],
    },
    {
      name: "Facturacion",
      rows: [
        [null, null, "Clave", null, "Precio"],
        [],
        [1, "PZA", " AA ", "Articulo A", 10, 10],
        [1, "PZA", "BB", "Articulo B", 20, 20],
        [],
        [1, "PZA", "CC", "Articulo C", 30, 30],
        [1, "PZA", "DD", "Articulo D", 40, 40],
        [null, null, "Observaciones", null, null],
      ],
    },
  ]);

  const system_path = await writeWorkbookFile("xlsx", [
    {
      name: "Resumen",
      rows: [
        ["Clave", "Descripcion", "Unidad", "Lista", "Precio"],
      ],
    },
    {
      name: "Base 1",
      rows: [
        ["AA", "Articulo A", "PZA", 100, 10],
        ["BB", "Articulo B", "PZA", 100, 22],
      ],
    },
    {
      name: "Base 2",
      rows: [
        ["CC", "Articulo C", "PZA", 100, 30],
        ["CC", "Articulo C duplicado", "PZA", 100, 31],
      ],
    },
  ]);

  try {
    const invoice_rows = await loadInvoiceRows(invoice_path);
    const system_rows = await loadSystemRows(system_path, 0);
    const report = reviewInvoice(
      invoice_path,
      invoice_rows,
      [system_path],
      system_rows,
    );

    assertEquals(invoice_rows.map((row) => row.key), ["AA", "BB", "CC", "DD"]);
    assertEquals(system_rows.map((row) => row.source_sheet), [
      "Base 1",
      "Base 1",
      "Base 2",
      "Base 2",
    ]);
    assertEquals(report.summary.total_rows_reviewed, 4);
    assertEquals(report.summary.total_price_mismatches, 1);
    assertEquals(report.summary.total_missing_keys, 1);
    assertEquals(report.summary.total_multiple_matches, 1);
    assertEquals(report.price_mismatches[0].key, "BB");
    assertEquals(report.missing_keys[0], "DD");
    assertEquals(report.multiple_matches[0].key, "CC");
  } finally {
    await Deno.remove(invoice_path);
    await Deno.remove(system_path);
  }
});

Deno.test("evita un falso cero cuando hay claves pero ningun precio valido", async () => {
  const invoice_path = await writeWorkbookFile("xls", [
    {
      name: "Report",
      rows: [
        [null, null, "Clave", null, "Precio"],
        [1, "PZA", "AA", "Articulo A", "sin precio"],
      ],
    },
  ]);

  try {
    await assertRejects(
      () => loadInvoiceRows(invoice_path),
      ValidationError,
      "No se encontraron filas utilizables",
    );
  } finally {
    await Deno.remove(invoice_path);
  }
});

Deno.test("aplica sensitivity sobre archivos .xls y .xlsx reales", async () => {
  const invoice_path = await writeWorkbookFile("xls", [
    {
      name: "Facturacion",
      rows: [
        [null, null, "Clave", null, "Precio"],
        [1, "PZA", "AA", "Articulo A", 100.004],
        [1, "PZA", "BB", "Articulo B", 99.99],
        [1, "PZA", "CC", "Articulo C", 99],
        [1, "PZA", "DD", "Articulo D", 50],
      ],
    },
  ]);

  const system_path = await writeWorkbookFile("xlsx", [
    {
      name: "Base",
      rows: [
        ["Clave", "Descripcion", "Unidad", "Lista", "Precio"],
        ["AA", "Articulo A", "PZA", 100, 100],
        ["BB", "Articulo B", "PZA", 100, 100],
        ["CC", "Articulo C", "PZA", 100, 100],
      ],
    },
  ]);

  try {
    const invoice_rows = await loadInvoiceRows(invoice_path);
    const system_rows = await loadSystemRows(system_path, 0);
    const report = reviewInvoice(
      invoice_path,
      invoice_rows,
      [system_path],
      system_rows,
      {
        sensitivity: -1,
      },
    );

    assertEquals(report.summary.total_rows_reviewed, 4);
    assertEquals(report.summary.total_price_mismatches, 1);
    assertEquals(report.summary.total_missing_keys, 1);
    assertEquals(report.summary.total_multiple_matches, 0);
    assertEquals(report.price_mismatches.map((mismatch) => mismatch.key), [
      "CC",
    ]);
    assertEquals(report.missing_keys, ["DD"]);
  } finally {
    await Deno.remove(invoice_path);
    await Deno.remove(system_path);
  }
});

async function writeWorkbookFile(
  book_type: "xls" | "xlsx",
  sheets: Array<{ name: string; rows: unknown[][] }>,
): Promise<string> {
  const workbook = XLSX.utils.book_new();

  for (const sheet_definition of sheets) {
    const sheet = XLSX.utils.aoa_to_sheet(sheet_definition.rows);
    XLSX.utils.book_append_sheet(workbook, sheet, sheet_definition.name);
  }

  const file_path = await Deno.makeTempFile({
    suffix: `.${book_type}`,
  });
  const workbook_bytes = XLSX.write(workbook, {
    type: "buffer",
    bookType: book_type,
  });

  await Deno.writeFile(file_path, new Uint8Array(workbook_bytes));

  return file_path;
}
