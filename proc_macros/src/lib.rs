use proc_macro::TokenStream;

pub(crate) mod cargo_lock_db;

/// Runs the `Cargo.lock` query at compile time.
///
/// We want to allow any syntax that is valid for `Lockfile` in `cargo-lockfile`. If your normal "query" against the lockfile was this:
/// ```rust
/// lockfile.some.query()
/// ```
///
/// Then you would translate it to `lockfile!` like this:
/// ```rust
/// lockfile!(some.query())
/// ```
///
/// Note that this macro statically loads in the project's `Cargo.lock`, so if there are multiple invocations of this macro, then it will only read in the project's `Cargo.lock` once per crate.
#[proc_macro]
pub fn lockfile(_lockfile_query: TokenStream) -> TokenStream {
    todo!()
}

#[cfg(test)]
mod tests {
    use cargo_lock::Lockfile;
    use cargo_lock_reader_core::utils::find_cargo_lock_file_path;

    #[test]
    fn read_project_lock_file() {
        let l_file_path = find_cargo_lock_file_path().unwrap();
        let l_file = Lockfile::load(l_file_path);

        assert!(l_file.is_ok());
    }
}
