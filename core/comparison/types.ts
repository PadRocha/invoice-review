import type { SystemRow } from "@interfaces/system_row.interface.ts";

export interface ReviewInvoiceConfig {
  sensitivity?: number;
}

export type MatchResolution =
  | {
    kind: "found";
    row: SystemRow;
  }
  | {
    kind: "not_found";
  }
  | {
    kind: "multiple";
    rows: SystemRow[];
  };
