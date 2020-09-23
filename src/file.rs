use crate::errors::GbRepoError;
use crate::models::Repository;
use std::fs::File;
use std::io::Write;

pub(crate) fn create_json_file(path: &str, repos: &[Repository]) -> Result<(), GbRepoError> {
    serde_json::to_writer_pretty(&File::create(format!("{}.json", path))?, repos)?;
    Ok(())
}

pub(crate) fn create_toml_file(path: &str, repos: &[Repository]) -> Result<(), GbRepoError> {
    let string = toml::ser::to_string_pretty(repos)?;
    let mut file = File::create(format!("{}.toml", path))?;
    file.write_all(&string.into_bytes())?;
    Ok(())
}
