import type { SystemRow } from "@interfaces/system_row.interface.ts";
import type { MatchResolution } from "./types.ts";

export function resolveSystemMatch(
  key: string,
  system_rows_by_file: Map<string, SystemRow[]>,
  system_priority_files: string[],
): MatchResolution {
  for (const system_file of system_priority_files) {
    const file_rows = system_rows_by_file.get(system_file) ?? [];
    const key_matches = file_rows.filter((row) => row.key === key);

    if (key_matches.length === 0) {
      continue;
    }

    if (key_matches.length === 1) {
      return {
        kind: "found",
        row: key_matches[0],
      };
    }

    return {
      kind: "multiple",
      rows: key_matches,
    };
  }

  return {
    kind: "not_found",
  };
}
