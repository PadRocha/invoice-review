pub fn parse_number(value: &str) -> Option<f64> {
    let normalized = value
        .trim()
        .replace(['$', '%', ' ', '\t', '\n', '\r'], "")
        .replace(',', "");

    if normalized.is_empty() {
        return None;
    }

    normalized
        .parse::<f64>()
        .ok()
        .filter(|value| value.is_finite())
}

pub fn round_number(value: f64, decimals: u32) -> f64 {
    let factor = 10_f64.powi(decimals as i32);
    ((value + f64::EPSILON) * factor).round() / factor
}

pub fn format_number(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{:.0}", value)
    } else {
        let mut text = value.to_string();
        if text.contains('.') {
            while text.ends_with('0') && text.contains('.') {
                text.pop();
            }
            if text.ends_with('.') {
                text.pop();
            }
        }
        text
    }
}
