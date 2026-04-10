use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use super::{
    calculate_discounted_price, resolve_system_match, InvoiceRow, MatchResolution, MultipleMatch,
    PercentageDelta, PriceMismatch, ReviewInvoiceConfig, ReviewReport, ReviewSummary,
    SensitivityThreshold, SystemRow,
};

const PERCENTAGE_FORMULA: &str = "precio_usado / precio_sistema * 100 - 100";

pub fn review_invoice(
    invoice_file: &Path,
    invoice_rows: &[InvoiceRow],
    system_files: &[PathBuf],
    system_rows: &[SystemRow],
    config: ReviewInvoiceConfig,
) -> ReviewReport {
    let system_rows_by_file = group_rows_by_file(system_rows);
    let mut price_mismatches = Vec::new();
    let mut missing_keys = Vec::new();
    let mut multiple_matches = Vec::new();

    for invoice_row in invoice_rows {
        match resolve_system_match(&invoice_row.key, &system_rows_by_file, system_files) {
            MatchResolution::NotFound => missing_keys.push(invoice_row.key.clone()),
            MatchResolution::Multiple(rows) => {
                let mut unique_files = Vec::new();
                let mut seen = HashSet::new();
                for row in &rows {
                    if seen.insert(row.source_file.clone()) {
                        unique_files.push(row.source_file.clone());
                    }
                }

                multiple_matches.push(MultipleMatch {
                    key: invoice_row.key.clone(),
                    invoice_row_number: invoice_row.row_number,
                    system_files: unique_files,
                    system_rows: rows.into_iter().map(|row| row.row_number).collect(),
                });
            }
            MatchResolution::Found(system_row) => {
                let discounted_price =
                    calculate_discounted_price(invoice_row.price, &config.discounts);

                if discounted_price.adjusted_price == system_row.price {
                    continue;
                }

                let percentage_result =
                    PercentageDelta::from_prices(discounted_price.adjusted_price, system_row.price);

                if is_hidden_by_sensitivity(percentage_result, config.sensitivity) {
                    continue;
                }

                price_mismatches.push(PriceMismatch {
                    key: invoice_row.key.clone(),
                    invoice_price: discounted_price.original_price,
                    compared_invoice_price: discounted_price.adjusted_price,
                    system_price: system_row.price,
                    system_file: system_row.source_file.clone(),
                    system_sheet: system_row.source_sheet.clone(),
                    invoice_row_number: invoice_row.row_number,
                    system_row_number: system_row.row_number,
                    percentage_formula: Cow::Borrowed(PERCENTAGE_FORMULA),
                    percentage_result,
                });
            }
        }
    }

    let summary = ReviewSummary {
        total_rows_reviewed: invoice_rows.len(),
        total_price_mismatches: price_mismatches.len(),
        total_missing_keys: missing_keys.len(),
        total_multiple_matches: multiple_matches.len(),
    };

    ReviewReport {
        invoice_file: invoice_file.to_path_buf(),
        system_files: system_files.to_vec(),
        discounts: config.discounts,
        price_mismatches,
        missing_keys,
        multiple_matches,
        summary,
    }
}

fn group_rows_by_file(system_rows: &[SystemRow]) -> HashMap<PathBuf, Vec<&SystemRow>> {
    let mut grouped_rows = HashMap::new();

    for system_row in system_rows {
        grouped_rows
            .entry(system_row.source_file.clone())
            .or_insert_with(Vec::new)
            .push(system_row);
    }

    grouped_rows
}

fn is_hidden_by_sensitivity(
    percentage_result: PercentageDelta,
    sensitivity: Option<SensitivityThreshold>,
) -> bool {
    sensitivity.is_some_and(|threshold| {
        percentage_result.value() <= 0.0 && percentage_result.value() > threshold.value()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::comparison::{
        DiscountRate, ItemKey, Price, SensitivityThreshold, SpreadsheetRowNumber,
    };
    use crate::shared::utils::round_number;

    fn key(value: &str) -> ItemKey {
        ItemKey::new(value).unwrap()
    }

    fn price(value: f64) -> Price {
        Price::new(value).unwrap()
    }

    fn discount(value: f64) -> DiscountRate {
        DiscountRate::new(value).unwrap()
    }

    fn sensitivity(value: f64) -> SensitivityThreshold {
        SensitivityThreshold::new(value).unwrap()
    }

    fn sheet_row(value: usize) -> SpreadsheetRowNumber {
        SpreadsheetRowNumber::new(value).unwrap()
    }

    fn invoice_row(row_number: usize, key_value: &str, price_value: f64) -> InvoiceRow {
        InvoiceRow {
            row_number: sheet_row(row_number),
            key: key(key_value),
            price: price(price_value),
            source_file: PathBuf::from("/tmp/47088.xls"),
        }
    }

    fn system_row(row_number: usize, key_value: &str, price_value: f64, file: &str) -> SystemRow {
        SystemRow {
            row_number: sheet_row(row_number),
            key: key(key_value),
            price: price(price_value),
            source_file: PathBuf::from(file),
            source_sheet: "Hoja1".to_string(),
            priority_index: 0,
        }
    }

    #[test]
    fn percentage_variation_applies_formula() {
        assert_eq!(round_number((83.63 / 95.85) * 100.0 - 100.0, 2), -12.75);
    }

    #[test]
    fn reports_price_missing_and_multiple() {
        let invoice_rows = [
            invoice_row(2, "A", 10.0),
            invoice_row(3, "B", 10.0),
            invoice_row(4, "C", 10.0),
            invoice_row(5, "D", 10.0),
        ];
        let system_files = [
            PathBuf::from("/tmp/EAG.xlsx"),
            PathBuf::from("/tmp/ACC.xlsx"),
        ];
        let system_rows = [
            system_row(8, "A", 20.0, "/tmp/EAG.xlsx"),
            system_row(9, "C", 10.0, "/tmp/EAG.xlsx"),
            system_row(10, "D", 11.0, "/tmp/ACC.xlsx"),
            system_row(11, "D", 12.0, "/tmp/ACC.xlsx"),
        ];

        let report = review_invoice(
            Path::new("/tmp/47088.xls"),
            &invoice_rows[..],
            &system_files[..],
            &system_rows[..],
            ReviewInvoiceConfig::default(),
        );

        assert_eq!(report.summary.total_rows_reviewed, 4);
        assert_eq!(report.summary.total_price_mismatches, 1);
        assert_eq!(report.summary.total_missing_keys, 1);
        assert_eq!(report.summary.total_multiple_matches, 1);
        assert_eq!(report.discounts, Vec::<DiscountRate>::new());
        assert_eq!(report.price_mismatches[0].key, key("A"));
        assert_eq!(report.missing_keys[0], key("B"));
        assert_eq!(report.multiple_matches[0].key, key("D"));
    }

    #[test]
    fn applies_discount_and_sensitivity_on_adjusted_price() {
        let invoice_rows = [invoice_row(2, "A", 100.0), invoice_row(3, "B", 100.0)];
        let system_files = [PathBuf::from("/tmp/EAG.xlsx")];
        let system_rows = [
            system_row(10, "A", 95.0, "/tmp/EAG.xlsx"),
            system_row(11, "B", 100.0, "/tmp/EAG.xlsx"),
        ];

        let report = review_invoice(
            Path::new("/tmp/47088.xls"),
            &invoice_rows[..],
            &system_files[..],
            &system_rows[..],
            ReviewInvoiceConfig {
                discounts: vec![discount(5.5)],
                sensitivity: Some(sensitivity(-1.0)),
            },
        );

        assert_eq!(report.summary.total_price_mismatches, 1);
        assert_eq!(report.price_mismatches[0].key, key("B"));
        assert_eq!(
            report.price_mismatches[0].compared_invoice_price.value(),
            94.5
        );
        assert_eq!(report.price_mismatches[0].percentage_result.value(), -5.5);
    }
}
