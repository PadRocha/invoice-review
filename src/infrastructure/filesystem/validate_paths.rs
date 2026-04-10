use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use path_clean::PathClean;

use crate::shared::FileSystemError;

pub fn resolve_path(path: impl AsRef<Path>) -> Result<PathBuf, FileSystemError> {
    let path = path.as_ref();
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        env::current_dir()
            .map_err(|source| FileSystemError::ResolveCurrentDirectory {
                path: path.to_path_buf(),
                source,
            })?
            .join(path)
    };

    Ok(absolute.clean())
}

pub fn validate_readable_file(path: &Path) -> Result<(), FileSystemError> {
    let file = fs::File::open(path).map_err(|source| {
        if source.kind() == std::io::ErrorKind::NotFound {
            FileSystemError::FileNotFound {
                path: path.to_path_buf(),
                source,
            }
        } else {
            FileSystemError::OpenFile {
                path: path.to_path_buf(),
                source,
            }
        }
    })?;

    let metadata = file
        .metadata()
        .map_err(|source| FileSystemError::ReadMetadata {
            path: path.to_path_buf(),
            source,
        })?;

    if !metadata.is_file() {
        return Err(FileSystemError::NotAFile {
            path: path.to_path_buf(),
        });
    }

    Ok(())
}
