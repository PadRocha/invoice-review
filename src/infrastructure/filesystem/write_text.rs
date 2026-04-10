use std::fs;
use std::path::Path;

use super::ensure_parent_directory;
use crate::shared::FileSystemError;

pub fn write_text_file(
    path: &Path,
    content: &str,
    ensure_trailing_newline: bool,
) -> Result<(), FileSystemError> {
    ensure_parent_directory(path)?;

    let final_content = if !ensure_trailing_newline || content.ends_with('\n') {
        content.to_string()
    } else {
        format!("{}\n", content)
    };

    fs::write(path, final_content).map_err(|source| FileSystemError::WriteFile {
        path: path.to_path_buf(),
        source,
    })
}
