use std::io;
use std::path::PathBuf;

use thiserror::Error;

use crate::domain::spreadsheet::SpreadsheetColumn;

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    Cli(#[from] CliError),
    #[error(transparent)]
    FileSystem(#[from] FileSystemError),
    #[error(transparent)]
    Spreadsheet(#[from] SpreadsheetError),
    #[error("No se pudo serializar el reporte a JSON. {source}")]
    JsonSerialization {
        #[from]
        source: serde_json::Error,
    },
}

impl AppError {
    pub fn exit_code(&self) -> i32 {
        match self {
            Self::Cli(_) => 2,
            Self::FileSystem(_) | Self::Spreadsheet(_) | Self::JsonSerialization { .. } => 1,
        }
    }
}

#[derive(Debug, Error)]
pub enum CliError {
    #[error("Debes indicar la factura con `--invoice <ruta>` o `-i <ruta>`.")]
    MissingInvoice,
    #[error("Debes indicar al menos un archivo del sistema con `--system <ruta>` o `-s <ruta>`.")]
    MissingSystemFile,
    #[error("`{flag}` requiere una ruta.")]
    MissingPathValue { flag: &'static str },
    #[error("`--system` requiere al menos una ruta.")]
    MissingSystemPathValue,
    #[error(
        "`--sensitivity` requiere un valor numérico negativo, por ejemplo `--sensitivity -1`."
    )]
    InvalidSensitivity { raw_value: Option<String> },
    #[error(
        "`--discount` requiere un porcentaje numérico mayor a 0 y menor a 100, por ejemplo `--discount 19` o `--discount 5.5`."
    )]
    InvalidDiscount { raw_value: Option<String> },
    #[error("Argumento no reconocido: {argument}")]
    UnknownFlag { argument: String },
    #[error(
        "Argumento no reconocido: {argument}. Esta CLI no usa subcomandos. Usa directamente -i y -s."
    )]
    UnexpectedPositionalArgument { argument: String },
}

impl CliError {
    pub fn missing_path_value(flag: &'static str) -> Self {
        Self::MissingPathValue { flag }
    }

    pub fn invalid_sensitivity(raw_value: Option<&str>) -> Self {
        Self::InvalidSensitivity {
            raw_value: raw_value.map(ToOwned::to_owned),
        }
    }

    pub fn invalid_discount(raw_value: Option<&str>) -> Self {
        Self::InvalidDiscount {
            raw_value: raw_value.map(ToOwned::to_owned),
        }
    }
}

#[derive(Debug, Error)]
pub enum FileSystemError {
    #[error(
        "No se pudo obtener el directorio actual para resolver la ruta {}. {source}",
        path.display()
    )]
    ResolveCurrentDirectory {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("No existe el archivo: {}", path.display())]
    FileNotFound {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("No se pudo abrir el archivo: {}. {source}", path.display())]
    OpenFile {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("No se pudo inspeccionar el archivo: {}. {source}", path.display())]
    ReadMetadata {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("La ruta no es un archivo: {}", path.display())]
    NotAFile { path: PathBuf },
    #[error(
        "No se pudo crear el directorio padre {} para {}. {source}",
        parent.display(),
        path.display()
    )]
    CreateParentDirectory {
        path: PathBuf,
        parent: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("No se pudo escribir el archivo: {}. {source}", path.display())]
    WriteFile {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
}

#[derive(Debug, Error)]
pub enum SpreadsheetError {
    #[error("No se pudo abrir la hoja de cálculo: {}. {source}", path.display())]
    OpenWorkbook {
        path: PathBuf,
        #[source]
        source: calamine::Error,
    },
    #[error(
        "No se pudo leer la hoja `{sheet_name}` en {}. {source}",
        path.display()
    )]
    ReadSheet {
        path: PathBuf,
        sheet_name: String,
        #[source]
        source: calamine::Error,
    },
    #[error("El archivo {} no contiene hojas utilizables.", path.display())]
    NoUsableSheets { path: PathBuf },
    #[error(
        "No se encontraron filas utilizables en {} usando clave en la columna {} y precio en la columna {}.",
        path.display(),
        key_column,
        price_column
    )]
    NoUsableRows {
        path: PathBuf,
        key_column: SpreadsheetColumn,
        price_column: SpreadsheetColumn,
    },
}
