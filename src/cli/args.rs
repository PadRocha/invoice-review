use std::path::PathBuf;

use crate::domain::comparison::{DiscountRate, SensitivityThreshold};
use crate::shared::CliError;

pub const CLI_NAME: &str = "invrev";
pub const CLI_VERSION: &str = "0.2.0";

#[derive(Debug, Clone, PartialEq)]
pub enum CliOptions {
    Help,
    Version,
    Review(ReviewCliOptions),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReviewCliOptions {
    pub invoice_path: Option<PathBuf>,
    pub system_paths: Vec<PathBuf>,
    pub discounts: Vec<DiscountRate>,
    pub output_path: Option<PathBuf>,
    pub json_path: Option<PathBuf>,
    pub sensitivity: Option<SensitivityThreshold>,
}

pub fn parse_cli_args(args: Vec<String>) -> Result<CliOptions, CliError> {
    if args.is_empty() {
        return Ok(CliOptions::Help);
    }

    if args.iter().any(|arg| arg == "--version" || arg == "-v") {
        return Ok(CliOptions::Version);
    }

    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        return Ok(CliOptions::Help);
    }

    let mut options = ReviewCliOptions {
        invoice_path: None,
        system_paths: Vec::new(),
        discounts: Vec::new(),
        output_path: None,
        json_path: None,
        sensitivity: None,
    };

    let mut index = 0usize;
    while let Some(current_arg) = args.get(index).map(String::as_str) {
        let current_arg: &str = current_arg;

        match current_arg {
            "--invoice" => {
                options.invoice_path =
                    Some(PathBuf::from(next_path_value(&args, index, "--invoice")?));
                index += 2;
                continue;
            }
            "-i" => {
                options.invoice_path = Some(PathBuf::from(next_path_value(&args, index, "-i")?));
                index += 2;
                continue;
            }
            "--system" | "-s" => {
                options
                    .system_paths
                    .push(PathBuf::from(next_system_path_value(&args, index)?));
                index += 2;
                continue;
            }
            "--out" => {
                options.output_path = Some(PathBuf::from(next_path_value(&args, index, "--out")?));
                index += 2;
                continue;
            }
            "-o" => {
                options.output_path = Some(PathBuf::from(next_path_value(&args, index, "-o")?));
                index += 2;
                continue;
            }
            "--json" => {
                options.json_path = Some(PathBuf::from(next_path_value(&args, index, "--json")?));
                index += 2;
                continue;
            }
            "--sensitivity" => {
                options.sensitivity = Some(parse_sensitivity_value(
                    args.get(index + 1).map(String::as_str),
                )?);
                index += 2;
                continue;
            }
            "--discount" | "-d" => {
                options.discounts.push(parse_discount_value(
                    args.get(index + 1).map(String::as_str),
                )?);
                index += 2;
                continue;
            }
            _ => {}
        }

        if let Some(value) = current_arg.strip_prefix("--system=") {
            let paths = split_system_paths(value);
            if paths.is_empty() {
                return Err(CliError::MissingSystemPathValue);
            }
            options.system_paths.extend(paths);
            index += 1;
            continue;
        }

        if let Some(value) = current_arg.strip_prefix("-s=") {
            let paths = split_system_paths(value);
            if paths.is_empty() {
                return Err(CliError::MissingSystemPathValue);
            }
            options.system_paths.extend(paths);
            index += 1;
            continue;
        }

        if let Some(value) = current_arg.strip_prefix("--sensitivity=") {
            options.sensitivity = Some(parse_sensitivity_value(Some(value))?);
            index += 1;
            continue;
        }

        if let Some(value) = current_arg.strip_prefix("--discount=") {
            options.discounts.push(parse_discount_value(Some(value))?);
            index += 1;
            continue;
        }

        if let Some(value) = current_arg.strip_prefix("-d=") {
            options.discounts.push(parse_discount_value(Some(value))?);
            index += 1;
            continue;
        }

        if current_arg.starts_with('-') {
            return Err(CliError::UnknownFlag {
                argument: current_arg.to_owned(),
            });
        }

        Err(CliError::UnexpectedPositionalArgument {
            argument: current_arg.to_owned(),
        })?;
    }

    Ok(CliOptions::Review(options))
}
pub fn build_version_message() -> String {
    format!("{} {}", CLI_NAME, CLI_VERSION)
}

pub fn build_help_message() -> String {
    [
        build_version_message(),
        String::new(),
        "Uso:".to_string(),
        format!("  {} --invoice ./47088.xls --system ./EAG.xlsx", CLI_NAME),
        format!(
            "  {} --invoice ./47088.xls --system ./EAG.xlsx --system ./ACC.xlsx",
            CLI_NAME,
        ),
        format!(
            "  {} --invoice ./47088.xls --system ./EAG.xlsx --discount 19 --discount 12",
            CLI_NAME,
        ),
        format!(
            "  {} --invoice ./47088.xls --system ./EAG.xlsx --sensitivity -1",
            CLI_NAME,
        ),
        format!(
            "  {} -i ./47088.xls -s ./EAG.xlsx -o ./reporte.txt --json ./reporte.json",
            CLI_NAME,
        ),
        String::new(),
        "Desarrollo:".to_string(),
        "  cargo run -- -i ./47088.xls -s ./EAG.xlsx".to_string(),
        String::new(),
        "Opciones:".to_string(),
        "  -i, --invoice <ruta>    Archivo principal de factura".to_string(),
        "  -s, --system <ruta>     Archivo del sistema. Puede repetirse".to_string(),
        "      --system=a,b,c      Variante compacta separada por comas".to_string(),
        "  -d, --discount <n>      Descuento porcentual. Puede repetirse".to_string(),
        "  -o, --out <ruta>        Exporta el reporte en texto".to_string(),
        "      --json <ruta>       Exporta el reporte en JSON".to_string(),
        "      --sensitivity <n>   Oculta diferencias en el rango (n, 0] si n es negativo"
            .to_string(),
        "  -v, --version          Muestra la versión actual".to_string(),
        "  -h, --help              Muestra esta ayuda".to_string(),
        String::new(),
        "Reglas fijas:".to_string(),
        "  - Factura: clave en columna C, precio en columna E".to_string(),
        "  - Sistema: clave en columna A, precio en columna E".to_string(),
        "  - Fórmula de variación: (precio_usado / precio_sistema) * 100 - 100".to_string(),
    ]
    .join("\n")
}

fn split_system_paths(value: &str) -> Vec<PathBuf> {
    value
        .split(',')
        .map(str::trim)
        .filter(|entry| !entry.is_empty())
        .map(PathBuf::from)
        .collect()
}

fn parse_sensitivity_value(raw_value: Option<&str>) -> Result<SensitivityThreshold, CliError> {
    let parsed = raw_value
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .and_then(|value| value.parse::<f64>().ok())
        .and_then(SensitivityThreshold::new);

    parsed.ok_or_else(|| CliError::invalid_sensitivity(raw_value))
}

fn parse_discount_value(raw_value: Option<&str>) -> Result<DiscountRate, CliError> {
    let Some(raw_value) = raw_value.map(str::trim).filter(|value| !value.is_empty()) else {
        return Err(CliError::invalid_discount(raw_value));
    };

    let parsed = raw_value.parse::<f64>().ok().and_then(DiscountRate::new);
    match parsed {
        Some(value) => Ok(value),
        _ => Err(CliError::invalid_discount(Some(raw_value))),
    }
}

fn next_path_value<'a>(
    args: &'a [String],
    index: usize,
    flag: &'static str,
) -> Result<&'a str, CliError> {
    let value = args
        .get(index + 1)
        .map(String::as_str)
        .filter(|value| !value.is_empty() && !value.starts_with('-'));

    value.ok_or_else(|| CliError::missing_path_value(flag))
}

fn next_system_path_value(args: &[String], index: usize) -> Result<&str, CliError> {
    let value = args
        .get(index + 1)
        .map(String::as_str)
        .filter(|value| !value.is_empty() && !value.starts_with('-'));

    value.ok_or(CliError::MissingSystemPathValue)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn discount(value: f64) -> DiscountRate {
        DiscountRate::new(value).unwrap()
    }

    fn sensitivity(value: f64) -> SensitivityThreshold {
        SensitivityThreshold::new(value).unwrap()
    }

    #[test]
    fn parse_cli_args_accepts_direct_invocation_with_flags() {
        let options = parse_cli_args(
            vec![
                "-i",
                "./47088.xls",
                "-s",
                "./EAG.xlsx",
                "--system=./ACC.xlsx,./BSC.xlsx",
                "-d",
                "19",
                "--discount=12.5",
                "--sensitivity",
                "-1",
                "-o",
                "./reporte.txt",
                "--json",
                "./reporte.json",
            ]
            .into_iter()
            .map(ToOwned::to_owned)
            .collect(),
        )
        .unwrap();

        assert_eq!(
            options,
            CliOptions::Review(ReviewCliOptions {
                invoice_path: Some(PathBuf::from("./47088.xls")),
                system_paths: vec![
                    PathBuf::from("./EAG.xlsx"),
                    PathBuf::from("./ACC.xlsx"),
                    PathBuf::from("./BSC.xlsx"),
                ],
                discounts: vec![discount(19.0), discount(12.5)],
                sensitivity: Some(sensitivity(-1.0)),
                output_path: Some(PathBuf::from("./reporte.txt")),
                json_path: Some(PathBuf::from("./reporte.json")),
            }),
        );
    }

    #[test]
    fn parse_cli_args_accepts_version() {
        assert_eq!(
            parse_cli_args(vec!["--version".to_string()]).unwrap(),
            CliOptions::Version,
        );
        assert_eq!(
            parse_cli_args(vec!["-v".to_string()]).unwrap(),
            CliOptions::Version,
        );
    }

    #[test]
    fn parse_cli_args_accepts_compact_sensitivity() {
        let options = parse_cli_args(
            vec![
                "--invoice",
                "./47088.xls",
                "--system",
                "./EAG.xlsx",
                "--discount",
                "5.5",
                "--sensitivity=-0.5",
            ]
            .into_iter()
            .map(ToOwned::to_owned)
            .collect(),
        )
        .unwrap();

        assert_eq!(
            options,
            CliOptions::Review(ReviewCliOptions {
                invoice_path: Some(PathBuf::from("./47088.xls")),
                system_paths: vec![PathBuf::from("./EAG.xlsx")],
                discounts: vec![discount(5.5)],
                sensitivity: Some(sensitivity(-0.5)),
                output_path: None,
                json_path: None,
            }),
        );
    }

    #[test]
    fn parse_cli_args_rejects_bad_discount() {
        let error = parse_cli_args(
            vec![
                "--invoice",
                "./47088.xls",
                "--system",
                "./EAG.xlsx",
                "--discount",
                "abc",
            ]
            .into_iter()
            .map(ToOwned::to_owned)
            .collect(),
        )
        .unwrap_err();

        assert_eq!(
      error.to_string(),
      "`--discount` requiere un porcentaje numérico mayor a 0 y menor a 100, por ejemplo `--discount 19` o `--discount 5.5`."
    );
    }

    #[test]
    fn parse_cli_args_rejects_missing_invoice_path_before_next_flag() {
        let error = parse_cli_args(
            vec!["--invoice", "--system", "./EAG.xlsx"]
                .into_iter()
                .map(ToOwned::to_owned)
                .collect(),
        )
        .unwrap_err();

        assert_eq!(error.to_string(), "`--invoice` requiere una ruta.");
    }

    #[test]
    fn parse_cli_args_rejects_empty_compact_system_paths() {
        let error = parse_cli_args(
            vec!["--invoice", "./47088.xls", "--system="]
                .into_iter()
                .map(ToOwned::to_owned)
                .collect(),
        )
        .unwrap_err();

        assert_eq!(error.to_string(), "`--system` requiere al menos una ruta.");
    }

    #[test]
    fn help_message_keeps_direct_invocation() {
        let help = build_help_message();
        assert!(help.contains(
            "invrev -i ./47088.xls -s ./EAG.xlsx -o ./reporte.txt --json ./reporte.json"
        ));
        assert!(help.contains("--discount <n>"));
        assert!(help.contains(&build_version_message()));
        assert!(help.contains("precio_usado / precio_sistema"));
    }
}
