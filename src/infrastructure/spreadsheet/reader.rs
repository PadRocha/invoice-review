use std::path::Path;

use calamine::{open_workbook_auto, Data, Reader};

use crate::domain::comparison::{InvoiceRow, ItemKey, Price, SpreadsheetRowNumber, SystemRow};
use crate::domain::spreadsheet::{
    SheetReadOptions, INVOICE_KEY_COLUMN, INVOICE_PRICE_COLUMN, SYSTEM_KEY_COLUMN,
    SYSTEM_PRICE_COLUMN,
};
use crate::shared::utils::parse_number;
use crate::shared::SpreadsheetError;

pub fn read_workbook(
    path: &Path,
) -> Result<calamine::Sheets<std::io::BufReader<std::fs::File>>, SpreadsheetError> {
    open_workbook_auto(path).map_err(|source| SpreadsheetError::OpenWorkbook {
        path: path.to_path_buf(),
        source,
    })
}

pub fn load_invoice_rows(path: &Path) -> Result<Vec<InvoiceRow>, SpreadsheetError> {
    let workbook = read_workbook(path)?;
    read_invoice_rows(
        workbook,
        path,
        SheetReadOptions {
            key_column: INVOICE_KEY_COLUMN,
            price_column: INVOICE_PRICE_COLUMN,
            priority_index: 0,
        },
    )
}

pub fn load_system_rows(
    path: &Path,
    priority_index: usize,
) -> Result<Vec<SystemRow>, SpreadsheetError> {
    let workbook = read_workbook(path)?;
    read_system_rows(
        workbook,
        path,
        SheetReadOptions {
            key_column: SYSTEM_KEY_COLUMN,
            price_column: SYSTEM_PRICE_COLUMN,
            priority_index,
        },
    )
}

fn read_invoice_rows(
    mut workbook: calamine::Sheets<std::io::BufReader<std::fs::File>>,
    source_file: &Path,
    options: SheetReadOptions,
) -> Result<Vec<InvoiceRow>, SpreadsheetError> {
    let rows = read_rows_from_workbook(&mut workbook, source_file, &options)?;
    Ok(rows
        .into_iter()
        .map(|entry| InvoiceRow {
            row_number: entry.row_number,
            key: entry.key,
            price: entry.price,
            source_file: source_file.to_path_buf(),
        })
        .collect())
}

fn read_system_rows(
    mut workbook: calamine::Sheets<std::io::BufReader<std::fs::File>>,
    source_file: &Path,
    options: SheetReadOptions,
) -> Result<Vec<SystemRow>, SpreadsheetError> {
    let rows = read_rows_from_workbook(&mut workbook, source_file, &options)?;
    Ok(rows
        .into_iter()
        .map(|entry| SystemRow {
            row_number: entry.row_number,
            key: entry.key,
            price: entry.price,
            source_file: source_file.to_path_buf(),
            source_sheet: entry.sheet_name,
            priority_index: options.priority_index,
        })
        .collect())
}

#[derive(Debug, Clone)]
struct ParsedSheetRow {
    row_number: SpreadsheetRowNumber,
    key: ItemKey,
    price: Price,
    sheet_name: String,
}

fn read_rows_from_workbook(
    workbook: &mut calamine::Sheets<std::io::BufReader<std::fs::File>>,
    source_file: &Path,
    options: &SheetReadOptions,
) -> Result<Vec<ParsedSheetRow>, SpreadsheetError> {
    let sheet_names = workbook.sheet_names().to_vec();
    if sheet_names.is_empty() {
        return Err(SpreadsheetError::NoUsableSheets {
            path: source_file.to_path_buf(),
        });
    }

    let mut rows = Vec::new();
    let mut candidate_row_count = 0usize;
    let mut usable_sheet_found = false;

    for sheet_name in sheet_names {
        let range = workbook.worksheet_range(&sheet_name).map_err(|source| {
            SpreadsheetError::ReadSheet {
                path: source_file.to_path_buf(),
                sheet_name: sheet_name.clone(),
                source,
            }
        })?;

        if range.is_empty() {
            continue;
        }

        usable_sheet_found = true;
        let key_column_index = options.key_column.zero_based_index();
        let price_column_index = options.price_column.zero_based_index();

        for (row_index, row) in range.rows().enumerate() {
            let spreadsheet_row_number = row_index + 1;
            let raw_key = row
                .get(key_column_index)
                .map(cell_to_text)
                .unwrap_or_default();
            let raw_price = row.get(price_column_index).map(cell_to_text);
            let Some(key) = ItemKey::new(raw_key) else {
                continue;
            };

            candidate_row_count += 1;
            let Some(price_raw) = raw_price else {
                continue;
            };
            let Some(price) = parse_number(&price_raw).and_then(Price::new) else {
                continue;
            };

            rows.push(ParsedSheetRow {
                row_number: SpreadsheetRowNumber::new(spreadsheet_row_number)
                    .expect("spreadsheet rows are 1-based"),
                key,
                price,
                sheet_name: sheet_name.clone(),
            });
        }
    }

    if !usable_sheet_found {
        return Err(SpreadsheetError::NoUsableSheets {
            path: source_file.to_path_buf(),
        });
    }

    if rows.is_empty() && candidate_row_count > 0 {
        return Err(SpreadsheetError::NoUsableRows {
            path: source_file.to_path_buf(),
            key_column: options.key_column,
            price_column: options.price_column,
        });
    }

    Ok(rows)
}

fn cell_to_text(cell: &Data) -> String {
    match cell {
        Data::String(value) => value.clone(),
        _ => cell.to_string(),
    }
}
