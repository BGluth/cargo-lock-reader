use std::{
    fs::{self, DirEntry},
    io,
    path::PathBuf,
    str::FromStr,
};

use thiserror::Error;

pub type LockFileSearchResult<T> = Result<T, LockFileSearchError>;

#[derive(Debug, Error)]
pub enum LockFileSearchError {
    #[error("Unable to find the project's `Cargo.lock` in \"{0}\".")]
    UnableToFindLockFile(String),

    #[error(transparent)]
    Io(#[from] io::Error),
}

pub fn find_cargo_lock_file_path() -> LockFileSearchResult<PathBuf> {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");

    let manifest_dir_path_buf = PathBuf::from_str(manifest_dir).unwrap();

    // Internal error type is `Infallible`, so we're good to use `unwrap()` here.
    let mut curr_dir = manifest_dir_path_buf.as_path();

    while let Some(parent_dir) = curr_dir.parent() {
        for entry in fs::read_dir(curr_dir)? {
            let entry = entry?;

            if dir_entry_is_cargo_lockfile(&entry)? {
                return Ok(entry.path());
            }
        }

        curr_dir = parent_dir;
    }

    Err(LockFileSearchError::UnableToFindLockFile(
        manifest_dir.to_string(),
    ))
}

fn dir_entry_is_cargo_lockfile(e: &DirEntry) -> Result<bool, io::Error> {
    Ok(e.file_name()
        .to_str()
        .map_or(false, |str| str == "Cargo.lock")
        && e.file_type()?.is_file())
}

#[cfg(test)]
mod tests {
    use super::find_cargo_lock_file_path;

    #[test]
    fn find_cargo_lock_path_works() {
        let res = find_cargo_lock_file_path();
        assert!(res.is_ok())
    }
}
