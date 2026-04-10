use std::borrow::Cow;
use std::fmt::{self, Display, Formatter};
use std::num::NonZeroUsize;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::shared::utils::calculate_percentage_variation;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ItemKey(String);

impl ItemKey {
    pub fn new(value: impl Into<String>) -> Option<Self> {
        let value = value.into();
        let normalized = value.trim();
        (!normalized.is_empty()).then(|| Self(normalized.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for ItemKey {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Display for ItemKey {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Price(f64);

impl Price {
    pub fn new(value: f64) -> Option<Self> {
        value.is_finite().then_some(Self(value))
    }

    pub const fn value(self) -> f64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DiscountRate(f64);

impl DiscountRate {
    pub fn new(value: f64) -> Option<Self> {
        (value.is_finite() && value > 0.0 && value < 100.0).then_some(Self(value))
    }

    pub const fn value(self) -> f64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SensitivityThreshold(f64);

impl SensitivityThreshold {
    pub fn new(value: f64) -> Option<Self> {
        (value.is_finite() && value < 0.0).then_some(Self(value))
    }

    pub const fn value(self) -> f64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PercentageDelta(f64);

impl PercentageDelta {
    pub const fn new(value: f64) -> Self {
        Self(value)
    }

    pub fn from_prices(compared_invoice_price: Price, system_price: Price) -> Self {
        Self(calculate_percentage_variation(
            compared_invoice_price.value(),
            system_price.value(),
        ))
    }

    pub const fn value(self) -> f64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SpreadsheetRowNumber(NonZeroUsize);

impl SpreadsheetRowNumber {
    pub fn new(value: usize) -> Option<Self> {
        NonZeroUsize::new(value).map(Self)
    }

    pub const fn get(self) -> usize {
        self.0.get()
    }
}

impl Display for SpreadsheetRowNumber {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.get())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InvoiceRow {
    pub row_number: SpreadsheetRowNumber,
    pub key: ItemKey,
    pub price: Price,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SystemRow {
    pub row_number: SpreadsheetRowNumber,
    pub key: ItemKey,
    pub price: Price,
    pub source_file: PathBuf,
    pub source_sheet: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PriceMismatch {
    pub key: ItemKey,
    pub invoice_price: Price,
    pub compared_invoice_price: Price,
    pub system_price: Price,
    pub system_file: PathBuf,
    pub system_sheet: String,
    pub invoice_row_number: SpreadsheetRowNumber,
    pub system_row_number: SpreadsheetRowNumber,
    pub percentage_formula: Cow<'static, str>,
    pub percentage_result: PercentageDelta,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MultipleMatch {
    pub key: ItemKey,
    pub invoice_row_number: SpreadsheetRowNumber,
    pub system_files: Vec<PathBuf>,
    pub system_rows: Vec<SpreadsheetRowNumber>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewSummary {
    pub total_rows_reviewed: usize,
    pub total_price_mismatches: usize,
    pub total_missing_keys: usize,
    pub total_multiple_matches: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReviewReport {
    pub invoice_file: PathBuf,
    pub system_files: Vec<PathBuf>,
    pub discounts: Vec<DiscountRate>,
    pub price_mismatches: Vec<PriceMismatch>,
    pub missing_keys: Vec<ItemKey>,
    pub multiple_matches: Vec<MultipleMatch>,
    pub summary: ReviewSummary,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ReviewInvoiceConfig {
    pub discounts: Vec<DiscountRate>,
    pub sensitivity: Option<SensitivityThreshold>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MatchResolution<'a> {
    Found(&'a SystemRow),
    NotFound,
    Multiple(Vec<&'a SystemRow>),
}
