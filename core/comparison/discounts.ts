export interface DiscountedPrice {
  original_price: number;
  discounts: number[];
  adjusted_price: number;
}

export function calculateDiscountedPrice(
  original_price: number,
  discounts: number[],
): DiscountedPrice {
  const adjusted_price = discounts.reduce(
    (current_price, discount) => current_price * (1 - discount / 100),
    original_price,
  );

  return {
    original_price,
    discounts: [...discounts],
    adjusted_price,
  };
}
