use std::fs;
use std::path::Path;

use crate::shared::FileSystemError;

pub fn ensure_parent_directory(path: &Path) -> Result<(), FileSystemError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|source| FileSystemError::CreateParentDirectory {
            path: path.to_path_buf(),
            parent: parent.to_path_buf(),
            source,
        })?;
    }

    Ok(())
}
