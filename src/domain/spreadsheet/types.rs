use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpreadsheetColumn {
    A,
    C,
    E,
}

impl SpreadsheetColumn {
    pub const fn zero_based_index(self) -> usize {
        match self {
            Self::A => 0,
            Self::C => 2,
            Self::E => 4,
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::A => "A",
            Self::C => "C",
            Self::E => "E",
        }
    }
}

impl Display for SpreadsheetColumn {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.label())
    }
}

pub const INVOICE_KEY_COLUMN: SpreadsheetColumn = SpreadsheetColumn::C;
pub const INVOICE_PRICE_COLUMN: SpreadsheetColumn = SpreadsheetColumn::E;
pub const SYSTEM_KEY_COLUMN: SpreadsheetColumn = SpreadsheetColumn::A;
pub const SYSTEM_PRICE_COLUMN: SpreadsheetColumn = SpreadsheetColumn::E;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SheetReadOptions {
    pub key_column: SpreadsheetColumn,
    pub price_column: SpreadsheetColumn,
    pub priority_index: usize,
}
