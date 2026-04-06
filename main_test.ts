import { assertEquals } from "@std/assert";
import { reviewInvoice } from "@comparison";
import { buildInvoiceRow, buildSystemRow } from "@models";
import { calculatePercentageVariation } from "@utils/percentage.ts";

Deno.test("calculatePercentageVariation aplica la fórmula requerida", () => {
  const result = calculatePercentageVariation(83.63, 95.85);
  assertEquals(result, -12.75);
});

Deno.test("reviewInvoice reporta precio incorrecto, faltante y múltiple", () => {
  const report = reviewInvoice(
    "/tmp/47088.xls",
    [
      buildInvoiceRow(2, "A", 10, "/tmp/47088.xls"),
      buildInvoiceRow(3, "B", 10, "/tmp/47088.xls"),
      buildInvoiceRow(4, "C", 10, "/tmp/47088.xls"),
      buildInvoiceRow(5, "D", 10, "/tmp/47088.xls"),
    ],
    ["/tmp/EAG.xlsx", "/tmp/ACC.xlsx"],
    [
      buildSystemRow(8, "A", 20, "/tmp/EAG.xlsx", "Hoja1", 0),
      buildSystemRow(9, "C", 10, "/tmp/EAG.xlsx", "Hoja1", 0),
      buildSystemRow(10, "D", 11, "/tmp/ACC.xlsx", "Hoja1", 1),
      buildSystemRow(11, "D", 12, "/tmp/ACC.xlsx", "Hoja1", 1),
    ],
  );

  assertEquals(report.summary.total_rows_reviewed, 4);
  assertEquals(report.summary.total_price_mismatches, 1);
  assertEquals(report.summary.total_missing_keys, 1);
  assertEquals(report.summary.total_multiple_matches, 1);
  assertEquals(report.price_mismatches[0].key, "A");
  assertEquals(report.missing_keys[0], "B");
  assertEquals(report.multiple_matches[0].key, "D");
});

Deno.test("reviewInvoice mantiene el comportamiento actual sin sensitivity", () => {
  const report = reviewInvoice(
    "/tmp/47088.xls",
    [
      buildInvoiceRow(2, "A", 100.004, "/tmp/47088.xls"),
    ],
    ["/tmp/EAG.xlsx"],
    [
      buildSystemRow(8, "A", 100, "/tmp/EAG.xlsx", "Hoja1", 0),
    ],
  );

  assertEquals(report.summary.total_rows_reviewed, 1);
  assertEquals(report.summary.total_price_mismatches, 1);
  assertEquals(report.price_mismatches[0].key, "A");
  assertEquals(report.price_mismatches[0].percentage_result, 0);
});

Deno.test("reviewInvoice oculta diferencias en (-1, 0] con sensitivity -1", () => {
  const report = reviewInvoice(
    "/tmp/47088.xls",
    [
      buildInvoiceRow(2, "A", 100.004, "/tmp/47088.xls"),
      buildInvoiceRow(3, "B", 99.99, "/tmp/47088.xls"),
      buildInvoiceRow(4, "C", 99.5, "/tmp/47088.xls"),
      buildInvoiceRow(5, "D", 99.01, "/tmp/47088.xls"),
      buildInvoiceRow(6, "E", 99, "/tmp/47088.xls"),
      buildInvoiceRow(7, "F", 98.5, "/tmp/47088.xls"),
      buildInvoiceRow(8, "G", 10, "/tmp/47088.xls"),
      buildInvoiceRow(9, "H", 10, "/tmp/47088.xls"),
    ],
    ["/tmp/EAG.xlsx"],
    [
      buildSystemRow(10, "A", 100, "/tmp/EAG.xlsx", "Hoja1", 0),
      buildSystemRow(11, "B", 100, "/tmp/EAG.xlsx", "Hoja1", 0),
      buildSystemRow(12, "C", 100, "/tmp/EAG.xlsx", "Hoja1", 0),
      buildSystemRow(13, "D", 100, "/tmp/EAG.xlsx", "Hoja1", 0),
      buildSystemRow(14, "E", 100, "/tmp/EAG.xlsx", "Hoja1", 0),
      buildSystemRow(15, "F", 100, "/tmp/EAG.xlsx", "Hoja1", 0),
      buildSystemRow(16, "H", 11, "/tmp/EAG.xlsx", "Hoja1", 0),
      buildSystemRow(17, "H", 12, "/tmp/EAG.xlsx", "Hoja1", 0),
    ],
    {
      sensitivity: -1,
    },
  );

  assertEquals(report.summary.total_rows_reviewed, 8);
  assertEquals(report.summary.total_price_mismatches, 2);
  assertEquals(report.summary.total_missing_keys, 1);
  assertEquals(report.summary.total_multiple_matches, 1);
  assertEquals(report.price_mismatches.map((mismatch) => mismatch.key), [
    "E",
    "F",
  ]);
  assertEquals(
    report.price_mismatches.map((mismatch) => mismatch.percentage_result),
    [-1, -1.5],
  );
  assertEquals(report.missing_keys, ["G"]);
  assertEquals(report.multiple_matches[0].key, "H");
});
