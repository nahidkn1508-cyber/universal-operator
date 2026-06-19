// src/utils.rs
use std::fs;
use std::io::{self, ErrorKind};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::error::{FileError, Result};

/// Maps an `io::Error` into a `FileError`, optionally including a path or description.
pub(crate) fn map_err_with_path(
    e: io::Error,
    context: impl std::fmt::Display,
) -> FileError {
    let ctx_path = context.to_string();
    match e.kind() {
        ErrorKind::NotFound => FileError::NotFound(PathBuf::from(ctx_path)),
        ErrorKind::AlreadyExists => FileError::AlreadyExists(PathBuf::from(ctx_path)),
        ErrorKind::PermissionDenied => FileError::PermissionDenied(PathBuf::from(ctx_path)),
        ErrorKind::DirectoryNotEmpty => FileError::DirectoryNotEmpty(PathBuf::from(ctx_path)),
        _ => FileError::Io {
            context: Some(ctx_path),
            source: e,
        },
    }
}

/// Recursively copies a directory tree using only `std::fs`.
pub(crate) fn copy_dir_all(from: &Path, to: &Path) -> Result<()> {
    fs::create_dir(to).map_err(|e| {
        if e.kind() == ErrorKind::AlreadyExists {
            FileError::AlreadyExists(to.to_path_buf())
        } else {
            map_err_with_path(e, to)
        }
    })?;

    for entry in fs::read_dir(from).map_err(|e| map_err_with_path(e, from))? {
        let entry = entry.map_err(|e| map_err_with_path(e, from))?;
        let file_type = entry
            .file_type()
            .map_err(|e| map_err_with_path(e, entry.path()))?;
        let from_path = entry.path();
        let to_path = to.join(entry.file_name());

        if file_type.is_dir() {
            copy_dir_all(&from_path, &to_path)?;
        } else {
            fs::copy(&from_path, &to_path).map_err(|e| {
                map_err_with_path(
                    e,
                    &format!("{} -> {}", from_path.display(), to_path.display()),
                )
            })?;
        }
    }
    Ok(())
}

static TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);

/// Generates a unique temporary file/directory name in the system's temp folder.
pub(crate) fn generate_temp_name(prefix: &str) -> PathBuf {
    let pid = std::process::id();
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let count = TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
    let name = format!("{prefix}{pid}_{nanos}_{count}");
    std::env::temp_dir().join(name)
}