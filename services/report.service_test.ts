import { assert, assertEquals, assertStringIncludes } from "@std/assert";
import { renderTextReport } from "./report.service.ts";

Deno.test("renderTextReport muestra precios incorrectos en formato compacto", () => {
  const text_report = renderTextReport({
    invoice_file: "/tmp/factura_47094.xls",
    system_files: ["/tmp/sistema_EAG.xlsx"],
    price_mismatches: [
      {
        key: "SOPEAG1062",
        invoice_price: 328.185,
        system_price: 328.18,
        system_file: "/tmp/sistema_EAG.xlsx",
        system_sheet: "Report",
        invoice_row_number: 2,
        system_row_number: 595,
        percentage_formula: "precio_factura / precio_sistema * 100 - 100",
        percentage_result: 0,
      },
      {
        key: "SOPEAG1112",
        invoice_price: 127.485,
        system_price: 127.48,
        system_file: "/tmp/sistema_EAG.xlsx",
        system_sheet: "Report",
        invoice_row_number: 4,
        system_row_number: 623,
        percentage_formula: "precio_factura / precio_sistema * 100 - 100",
        percentage_result: 0,
      },
    ],
    missing_keys: [],
    multiple_matches: [],
    summary: {
      total_rows_reviewed: 2,
      total_price_mismatches: 2,
      total_missing_keys: 0,
      total_multiple_matches: 0,
    },
  });

  assertStringIncludes(
    text_report,
    [
      "1. Precios incorrectos",
      "- SOPEAG1062",
      "  - precio en factura: 328.185",
      "  - precio en sistema: 328.18",
      "  - archivo del sistema: sistema_EAG.xlsx",
      "  - resultado porcentual: 0%",
      "",
      "- SOPEAG1112",
    ].join("\n"),
  );
  assert(!text_report.includes("fila en factura"));
  assert(!text_report.includes("hoja del sistema"));
  assert(!text_report.includes("fila en sistema"));
  assert(!text_report.includes("cálculo aplicado"));
  assertEquals(text_report.includes("archivo del sistema"), true);
});
