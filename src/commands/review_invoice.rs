use crate::application::review_invoice::{self, ReviewInvoiceRequest};
use crate::cli::ReviewCliOptions;
use crate::shared::{AppError, CliError};

pub fn execute(options: ReviewCliOptions) -> Result<String, AppError> {
    review_invoice::run(validate(options)?)
}

fn validate(options: ReviewCliOptions) -> Result<ReviewInvoiceRequest, CliError> {
    let Some(invoice_path) = options.invoice_path else {
        return Err(CliError::MissingInvoice);
    };

    if options.system_paths.is_empty() {
        return Err(CliError::MissingSystemFile);
    }

    Ok(ReviewInvoiceRequest {
        invoice_path,
        system_paths: options.system_paths,
        discounts: options.discounts,
        output_path: options.output_path,
        json_path: options.json_path,
        sensitivity: options.sensitivity,
    })
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::domain::comparison::{DiscountRate, SensitivityThreshold};

    use super::*;

    fn discount(value: f64) -> DiscountRate {
        DiscountRate::new(value).unwrap()
    }

    fn sensitivity(value: f64) -> SensitivityThreshold {
        SensitivityThreshold::new(value).unwrap()
    }

    #[test]
    fn validate_requires_invoice() {
        let error = validate(ReviewCliOptions {
            invoice_path: None,
            system_paths: vec![PathBuf::from("./EAG.xlsx")],
            discounts: vec![],
            output_path: None,
            json_path: None,
            sensitivity: None,
        })
        .unwrap_err();

        assert_eq!(
            error.to_string(),
            "Debes indicar la factura con `--invoice <ruta>` o `-i <ruta>`."
        );
    }

    #[test]
    fn validate_requires_at_least_one_system_file() {
        let error = validate(ReviewCliOptions {
            invoice_path: Some(PathBuf::from("./47088.xls")),
            system_paths: vec![],
            discounts: vec![],
            output_path: None,
            json_path: None,
            sensitivity: None,
        })
        .unwrap_err();

        assert_eq!(
            error.to_string(),
            "Debes indicar al menos un archivo del sistema con `--system <ruta>` o `-s <ruta>`."
        );
    }

    #[test]
    fn validate_translates_cli_options_into_application_request() {
        let request = validate(ReviewCliOptions {
            invoice_path: Some(PathBuf::from("./47088.xls")),
            system_paths: vec![PathBuf::from("./EAG.xlsx"), PathBuf::from("./ACC.xlsx")],
            discounts: vec![discount(19.0)],
            output_path: Some(PathBuf::from("./reporte.txt")),
            json_path: Some(PathBuf::from("./reporte.json")),
            sensitivity: Some(sensitivity(-1.0)),
        })
        .unwrap();

        assert_eq!(
            request,
            ReviewInvoiceRequest {
                invoice_path: PathBuf::from("./47088.xls"),
                system_paths: vec![PathBuf::from("./EAG.xlsx"), PathBuf::from("./ACC.xlsx")],
                discounts: vec![discount(19.0)],
                output_path: Some(PathBuf::from("./reporte.txt")),
                json_path: Some(PathBuf::from("./reporte.json")),
                sensitivity: Some(sensitivity(-1.0)),
            }
        );
    }
}
