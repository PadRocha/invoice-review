import { resolve } from "@std/path";
import type { ValidReviewCliOptions } from "@cli";
import { reviewInvoice } from "@comparison";
import { validateReadableFile } from "@services/filesystem/validate_paths.ts";
import { writeTextFile } from "@services/filesystem/write_text.ts";
import { renderTextReport } from "@services/report.service.ts";
import {
  loadInvoiceRows,
  loadSystemRows,
} from "@services/spreadsheet.service.ts";

export async function reviewInvoiceController(
  options: ValidReviewCliOptions,
): Promise<string> {
  const invoice_path = resolve(options.invoice_path);
  const system_paths = options.system_paths.map((path) => resolve(path));

  await validateReadableFile(invoice_path);

  for (const system_path of system_paths) {
    await validateReadableFile(system_path);
  }

  const invoice_rows = await loadInvoiceRows(invoice_path);
  const system_rows = (
    await Promise.all(
      system_paths.map((system_path, index) =>
        loadSystemRows(system_path, index)
      ),
    )
  ).flat();

  const report = reviewInvoice(
    invoice_path,
    invoice_rows,
    system_paths,
    system_rows,
    {
      discounts: options.discounts,
      sensitivity: options.sensitivity,
    },
  );

  const text_report = renderTextReport(report);

  if (options.output_path) {
    await writeTextFile(resolve(options.output_path), text_report);
  }

  if (options.json_path) {
    await writeTextFile(
      resolve(options.json_path),
      JSON.stringify(report, null, 2),
    );
  }

  return text_report;
}
