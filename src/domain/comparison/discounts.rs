use super::{DiscountRate, Price};

#[derive(Debug, Clone, PartialEq)]
pub struct DiscountedPrice {
    pub original_price: Price,
    pub discounts: Vec<DiscountRate>,
    pub adjusted_price: Price,
}

pub fn calculate_discounted_price(
    original_price: Price,
    discounts: &[DiscountRate],
) -> DiscountedPrice {
    let adjusted_price = discounts
        .iter()
        .fold(original_price.value(), |current, discount| {
            current * (1.0 - discount.value() / 100.0)
        });

    DiscountedPrice {
        original_price,
        discounts: discounts.to_vec(),
        adjusted_price: Price::new(adjusted_price)
            .expect("applying finite discounts to a finite price should stay finite"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn price(value: f64) -> Price {
        Price::new(value).unwrap()
    }

    fn discount(value: f64) -> DiscountRate {
        DiscountRate::new(value).unwrap()
    }

    #[test]
    fn keeps_original_price_without_discounts() {
        assert_eq!(
            calculate_discounted_price(price(530.0), &[]),
            DiscountedPrice {
                original_price: price(530.0),
                discounts: vec![],
                adjusted_price: price(530.0),
            },
        );
    }

    #[test]
    fn applies_successive_discounts() {
        let discounts = [discount(19.0), discount(12.0)];
        let result = calculate_discounted_price(price(530.0), &discounts[..]);
        assert_eq!(result.adjusted_price.value(), 377.784);
    }
}
