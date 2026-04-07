export interface PriceMismatch {
  key: string;
  invoice_price: number;
  compared_invoice_price: number;
  system_price: number;
  system_file: string;
  system_sheet: string;
  invoice_row_number: number;
  system_row_number: number;
  percentage_formula: string;
  percentage_result: number;
}
