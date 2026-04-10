use super::round_number;

pub fn calculate_percentage_variation(invoice_price: f64, system_price: f64) -> f64 {
    round_number((invoice_price / system_price) * 100.0 - 100.0, 2)
}
