use crate::domain::comparison::{review_invoice, ReviewInvoiceConfig};
use crate::domain::comparison::{DiscountRate, SensitivityThreshold};
use crate::infrastructure::filesystem::{resolve_path, validate_readable_file, write_text_file};
use crate::infrastructure::report::render_text_report;
use crate::infrastructure::spreadsheet::{load_invoice_rows, load_system_rows};
use crate::shared::AppError;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub struct ReviewInvoiceRequest {
    pub invoice_path: PathBuf,
    pub system_paths: Vec<PathBuf>,
    pub discounts: Vec<DiscountRate>,
    pub output_path: Option<PathBuf>,
    pub json_path: Option<PathBuf>,
    pub sensitivity: Option<SensitivityThreshold>,
}

pub fn run(request: ReviewInvoiceRequest) -> Result<String, AppError> {
    let invoice_path = resolve_path(&request.invoice_path)?;
    let system_paths = request
        .system_paths
        .iter()
        .map(resolve_path)
        .collect::<Result<Vec<_>, _>>()?;

    validate_readable_file(&invoice_path)?;
    for system_path in &system_paths {
        validate_readable_file(system_path)?;
    }

    let invoice_rows = load_invoice_rows(&invoice_path)?;
    let mut system_rows = Vec::new();
    for system_path in &system_paths {
        system_rows.extend(load_system_rows(system_path)?);
    }

    let report = review_invoice(
        &invoice_path,
        &invoice_rows,
        &system_paths,
        &system_rows,
        ReviewInvoiceConfig {
            discounts: request.discounts,
            sensitivity: request.sensitivity,
        },
    );

    let text_report = render_text_report(&report);

    if let Some(output_path) = request.output_path {
        let output_path = resolve_path(&output_path)?;
        write_text_file(&output_path, &text_report, true)?;
    }

    if let Some(json_path) = request.json_path {
        let json_path = resolve_path(&json_path)?;
        let content = serde_json::to_string_pretty(&report)?;
        write_text_file(&json_path, &content, true)?;
    }

    Ok(text_report)
}
