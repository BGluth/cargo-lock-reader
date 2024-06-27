use cargo_lock::Lockfile;
use cargo_lock_reader_core::utils::find_cargo_lock_file_path;
use lazy_static::lazy_static;

lazy_static! {
    // TODO: Handle these errors nicely somehow...
    static ref LOCK_DB: Lockfile = {
        let manifest_path =
            find_cargo_lock_file_path().expect("Unable to load in the project's `Cargo.lock`!");
        Lockfile::load(manifest_path).unwrap()
    };
}
