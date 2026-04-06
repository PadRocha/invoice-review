import { roundNumber } from "./number.ts";

export function calculatePercentageVariation(
  invoice_price: number,
  system_price: number,
): number {
  return roundNumber((invoice_price / system_price) * 100 - 100, 2);
}
