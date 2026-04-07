import { assertEquals } from "@std/assert";
import { calculateDiscountedPrice } from "./discounts.ts";

Deno.test("calculateDiscountedPrice mantiene el precio original sin descuentos", () => {
  const result = calculateDiscountedPrice(530, []);

  assertEquals(result, {
    original_price: 530,
    discounts: [],
    adjusted_price: 530,
  });
});

Deno.test("calculateDiscountedPrice aplica un descuento porcentual", () => {
  const result = calculateDiscountedPrice(530, [19]);

  assertEquals(result, {
    original_price: 530,
    discounts: [19],
    adjusted_price: 429.3,
  });
});

Deno.test("calculateDiscountedPrice aplica descuentos sucesivos", () => {
  const result = calculateDiscountedPrice(530, [19, 12]);

  assertEquals(result, {
    original_price: 530,
    discounts: [19, 12],
    adjusted_price: 377.784,
  });
});

Deno.test("calculateDiscountedPrice acepta descuentos decimales", () => {
  const result = calculateDiscountedPrice(100, [5.5]);

  assertEquals(result, {
    original_price: 100,
    discounts: [5.5],
    adjusted_price: 94.5,
  });
});
