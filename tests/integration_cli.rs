use std::path::{Path, PathBuf};

use invrev::cli::{build_version_message, parse_cli_args, CliOptions};
use invrev::domain::comparison::{
    review_invoice, InvoiceRow, ItemKey, PercentageDelta, Price, ReviewInvoiceConfig,
    SpreadsheetRowNumber, SystemRow,
};

fn key(value: &str) -> ItemKey {
    ItemKey::new(value).unwrap()
}

fn price(value: f64) -> Price {
    Price::new(value).unwrap()
}

fn row_number(value: usize) -> SpreadsheetRowNumber {
    SpreadsheetRowNumber::new(value).unwrap()
}

#[test]
fn main_like_version_flow_short_circuits_review() {
    let options = parse_cli_args(vec![
        "--version".to_string(),
        "-i".to_string(),
        "./factura-inexistente.xls".to_string(),
        "-s".to_string(),
        "./sistema-inexistente.xlsx".to_string(),
    ])
    .unwrap();

    assert_eq!(options, CliOptions::Version);
    assert_eq!(build_version_message(), "invrev 0.2.0");
}

#[test]
fn review_invoice_keeps_original_behavior_without_sensitivity() {
    let invoice_rows = [InvoiceRow {
        row_number: row_number(2),
        key: key("A"),
        price: price(100.004),
    }];
    let system_files = [PathBuf::from("/tmp/EAG.xlsx")];
    let system_rows = [SystemRow {
        row_number: row_number(8),
        key: key("A"),
        price: price(100.0),
        source_file: PathBuf::from("/tmp/EAG.xlsx"),
        source_sheet: "Hoja1".to_string(),
    }];

    let report = review_invoice(
        Path::new("/tmp/47088.xls"),
        &invoice_rows[..],
        &system_files[..],
        &system_rows[..],
        ReviewInvoiceConfig::default(),
    );

    assert_eq!(report.summary.total_price_mismatches, 1);
    assert_eq!(
        report.price_mismatches[0].percentage_result,
        PercentageDelta::new(0.0),
    );
}

#[test]
fn review_report_serialization_keeps_flat_json_shape() {
    let invoice_rows = [InvoiceRow {
        row_number: row_number(2),
        key: key("A"),
        price: price(100.0),
    }];
    let system_files = [PathBuf::from("/tmp/EAG.xlsx")];
    let system_rows = [SystemRow {
        row_number: row_number(8),
        key: key("A"),
        price: price(95.0),
        source_file: PathBuf::from("/tmp/EAG.xlsx"),
        source_sheet: "Hoja1".to_string(),
    }];

    let report = review_invoice(
        Path::new("/tmp/47088.xls"),
        &invoice_rows[..],
        &system_files[..],
        &system_rows[..],
        ReviewInvoiceConfig::default(),
    );

    let json = serde_json::to_value(&report).unwrap();

    assert_eq!(json["invoice_file"], "/tmp/47088.xls");
    assert_eq!(json["system_files"][0], "/tmp/EAG.xlsx");
    assert_eq!(json["price_mismatches"][0]["key"], "A");
    assert_eq!(json["price_mismatches"][0]["invoice_price"], 100.0);
    assert_eq!(json["price_mismatches"][0]["invoice_row_number"], 2);
    assert_eq!(json["price_mismatches"][0]["percentage_result"], 5.26);
}
