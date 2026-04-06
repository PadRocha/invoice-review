export function parseNumber(value: unknown): number | null {
  if (typeof value === "number" && Number.isFinite(value)) {
    return value;
  }

  if (typeof value !== "string") {
    return null;
  }

  const normalized_value = value
    .trim()
    .replaceAll(/[$,%\s]/g, "")
    .replaceAll(",", "");

  if (!normalized_value) {
    return null;
  }

  const parsed_value = Number(normalized_value);
  return Number.isFinite(parsed_value) ? parsed_value : null;
}

export function roundNumber(value: number, decimals = 2): number {
  const factor = 10 ** decimals;
  return Math.round((value + Number.EPSILON) * factor) / factor;
}

export function formatNumber(value: number): string {
  if (Number.isInteger(value)) {
    return value.toString();
  }

  return value.toString();
}
