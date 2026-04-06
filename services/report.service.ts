import { basename } from "@std/path";
import type { ReviewReport } from "@interfaces/review_report.interface.ts";
import { formatNumber } from "@utils/number.ts";

export function renderTextReport(report: ReviewReport): string {
  const lines: string[] = [];

  lines.push(`Factura: ${basename(report.invoice_file)}`);
  lines.push(
    `Sistema: ${
      report.system_files.map((entry) => basename(entry)).join(", ")
    }`,
  );
  lines.push("");
  lines.push("1. Precios incorrectos");

  if (report.price_mismatches.length === 0) {
    lines.push("- Ninguno");
  } else {
    for (
      let mismatch_index = 0;
      mismatch_index < report.price_mismatches.length;
      mismatch_index += 1
    ) {
      const mismatch = report.price_mismatches[mismatch_index];
      lines.push(`- ${mismatch.key}`);
      lines.push(
        `  - precio en factura: ${formatNumber(mismatch.invoice_price)}`,
      );
      lines.push(
        `  - precio en sistema: ${formatNumber(mismatch.system_price)}`,
      );
      lines.push(
        `  - archivo del sistema: ${basename(mismatch.system_file)}`,
      );
      lines.push(`  - resultado porcentual: ${mismatch.percentage_result}%`);

      if (mismatch_index < report.price_mismatches.length - 1) {
        lines.push("");
      }
    }
  }

  lines.push("");
  lines.push("2. Claves no encontradas");

  if (report.missing_keys.length === 0) {
    lines.push("- Ninguna");
  } else {
    for (const key of report.missing_keys) {
      lines.push(`- ${key}`);
    }
  }

  lines.push("");
  lines.push("3. Coincidencias múltiples");

  if (report.multiple_matches.length === 0) {
    lines.push("- Ninguna");
  } else {
    for (const match of report.multiple_matches) {
      lines.push(`- ${match.key}`);
      lines.push(`  - fila en factura: ${match.invoice_row_number}`);
      lines.push(
        `  - archivo(s) del sistema: ${
          match.system_files.map((entry) => basename(entry)).join(", ")
        }`,
      );
      lines.push(`  - fila(s) del sistema: ${match.system_rows.join(", ")}`);
    }
  }

  lines.push("");
  lines.push("4. Resumen final");
  lines.push(
    `- total de filas revisadas: ${report.summary.total_rows_reviewed}`,
  );
  lines.push(
    `- total de precios incorrectos: ${report.summary.total_price_mismatches}`,
  );
  lines.push(
    `- total de claves no encontradas: ${report.summary.total_missing_keys}`,
  );
  lines.push(
    `- total de coincidencias múltiples: ${report.summary.total_multiple_matches}`,
  );

  return lines.join("\n");
}
