use std::collections::HashMap;
use std::path::PathBuf;

use super::{ItemKey, MatchResolution, SystemRow};

pub fn resolve_system_match<'a>(
    key: &ItemKey,
    system_rows_by_file: &HashMap<&'a PathBuf, Vec<&'a SystemRow>>,
    system_priority_files: &[PathBuf],
) -> MatchResolution<'a> {
    for system_file in system_priority_files {
        let Some(file_rows) = system_rows_by_file.get(system_file) else {
            continue;
        };

        let key_matches = file_rows
            .iter()
            .copied()
            .filter(|row| row.key == *key)
            .collect::<Vec<_>>();

        if key_matches.is_empty() {
            continue;
        }

        if let [match_row] = key_matches.as_slice() {
            return MatchResolution::Found(match_row);
        }

        return MatchResolution::Multiple(key_matches);
    }

    MatchResolution::NotFound
}
