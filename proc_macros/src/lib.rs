pub(crate) mod cargo_lock_db;

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
