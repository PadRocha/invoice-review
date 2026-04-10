use std::path::Path;

use crate::domain::comparison::ReviewReport;
use crate::shared::utils::{format_number, round_number};

pub fn render_text_report(report: &ReviewReport) -> String {
    let mut lines = Vec::new();
    let has_discounts = !report.discounts.is_empty();

    lines.push(format!(
        "Factura: {}",
        basename(report.invoice_file.as_path())
    ));
    lines.push(format!(
        "Sistema: {}",
        report
            .system_files
            .iter()
            .map(|entry| basename(entry.as_path()))
            .collect::<Vec<_>>()
            .join(", "),
    ));
    lines.push(String::new());
    lines.push("1. Precios incorrectos".to_string());

    if report.price_mismatches.is_empty() {
        lines.push("- Ninguno".to_string());
    } else {
        for (index, mismatch) in report.price_mismatches.iter().enumerate() {
            lines.push(format!("- {}", mismatch.key));
            if has_discounts {
                lines.push(format!(
                    "  - precio original en factura: {}",
                    format_number(mismatch.invoice_price.value()),
                ));
                lines.push(format!(
                    "  - precio con descuentos aplicados: {}",
                    format_number(round_number(mismatch.compared_invoice_price.value(), 2)),
                ));
            } else {
                lines.push(format!(
                    "  - precio en factura: {}",
                    format_number(mismatch.invoice_price.value()),
                ));
            }
            lines.push(format!(
                "  - precio en sistema: {}",
                format_number(mismatch.system_price.value()),
            ));
            lines.push(format!(
                "  - archivo del sistema: {}",
                basename(mismatch.system_file.as_path()),
            ));
            lines.push(format!(
                "  - resultado porcentual: {}%",
                format_number(mismatch.percentage_result.value()),
            ));
            if index + 1 < report.price_mismatches.len() {
                lines.push(String::new());
            }
        }
    }

    lines.push(String::new());
    lines.push("2. Claves no encontradas".to_string());
    if report.missing_keys.is_empty() {
        lines.push("- Ninguna".to_string());
    } else {
        for key in &report.missing_keys {
            lines.push(format!("- {}", key));
        }
    }

    lines.push(String::new());
    lines.push("3. Coincidencias múltiples".to_string());
    if report.multiple_matches.is_empty() {
        lines.push("- Ninguna".to_string());
    } else {
        for entry in &report.multiple_matches {
            lines.push(format!("- {}", entry.key));
            lines.push(format!("  - fila en factura: {}", entry.invoice_row_number));
            lines.push(format!(
                "  - archivo(s) del sistema: {}",
                entry
                    .system_files
                    .iter()
                    .map(|path| basename(path.as_path()))
                    .collect::<Vec<_>>()
                    .join(", "),
            ));
            lines.push(format!(
                "  - fila(s) del sistema: {}",
                entry
                    .system_rows
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", "),
            ));
        }
    }

    lines.push(String::new());
    lines.push("4. Resumen final".to_string());
    lines.push(format!(
        "- total de filas revisadas: {}",
        report.summary.total_rows_reviewed,
    ));
    lines.push(format!(
        "- total de precios incorrectos: {}",
        report.summary.total_price_mismatches,
    ));
    lines.push(format!(
        "- total de claves no encontradas: {}",
        report.summary.total_missing_keys,
    ));
    lines.push(format!(
        "- total de coincidencias múltiples: {}",
        report.summary.total_multiple_matches,
    ));

    lines.join("\n")
}

fn basename(path: &Path) -> String {
    path.file_name()
        .map(|value| value.to_string_lossy().to_string())
        .unwrap_or_else(|| path.display().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Cow;
    use std::path::PathBuf;

    use crate::domain::comparison::{
        DiscountRate, ItemKey, PercentageDelta, Price, PriceMismatch, ReviewSummary,
        SpreadsheetRowNumber,
    };

    fn key(value: &str) -> ItemKey {
        ItemKey::new(value).unwrap()
    }

    fn price(value: f64) -> Price {
        Price::new(value).unwrap()
    }

    fn discount(value: f64) -> DiscountRate {
        DiscountRate::new(value).unwrap()
    }

    fn row_number(value: usize) -> SpreadsheetRowNumber {
        SpreadsheetRowNumber::new(value).unwrap()
    }

    #[test]
    fn shows_discount_context_only_when_present() {
        let report = ReviewReport {
            invoice_file: PathBuf::from("/tmp/factura_47094.xls"),
            system_files: vec![PathBuf::from("/tmp/sistema_EAG.xlsx")],
            discounts: vec![discount(19.0), discount(12.0)],
            price_mismatches: vec![PriceMismatch {
                key: key("SOPEAG1062"),
                invoice_price: price(530.0),
                compared_invoice_price: price(377.784),
                system_price: price(377.78),
                system_file: PathBuf::from("/tmp/sistema_EAG.xlsx"),
                system_sheet: "Report".to_string(),
                invoice_row_number: row_number(2),
                system_row_number: row_number(595),
                percentage_formula: Cow::Borrowed("precio_usado / precio_sistema * 100 - 100"),
                percentage_result: PercentageDelta::new(0.0),
            }],
            missing_keys: vec![],
            multiple_matches: vec![],
            summary: ReviewSummary {
                total_rows_reviewed: 1,
                total_price_mismatches: 1,
                total_missing_keys: 0,
                total_multiple_matches: 0,
            },
        };

        let text = render_text_report(&report);
        assert!(text.contains("  - precio original en factura: 530"));
        assert!(text.contains("  - precio con descuentos aplicados: 377.78"));
        assert!(!text.contains("  - precio en factura: 530"));
    }
}
