use std::{
    fs::{self, DirEntry},
    io,
    path::PathBuf,
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

pub(crate) fn find_cargo_lock_file() -> LockFileSearchResult<PathBuf> {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    println!("{}", manifest_dir);

    for entry in fs::read_dir(manifest_dir)? {
        let entry = entry?;

        println!("ENTRY: {:?}", entry);

        if dir_entry_is_cargo_lockfile(&entry)? {
            return Ok(entry.path());
        }
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
    use super::find_cargo_lock_file;

    #[test]
    fn find_cargo_lock_path_works() {
        let res = find_cargo_lock_file();

        println!("Res: {:?}", res);

        assert!(res.is_ok())
    }
}
