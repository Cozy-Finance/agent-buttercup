use eyre::{Context as _, Result};
use std::path::{Path, PathBuf};

/// Returns the canonical path string.
pub fn get_canonical_path_string(path: impl AsRef<Path>) -> Result<String> {
    Ok(dunce::canonicalize(path)
        .wrap_err("Could not canonicalize raw abi path.")?
        .to_str()
        .ok_or_else(|| eyre::eyre!("File name contains invalid UTF-8"))?
        .to_string())
}

/// Returns a list of absolute paths to all the json files under the root.
pub fn json_files(root: &impl AsRef<Path>) -> impl Iterator<Item = PathBuf> {
    walkdir::WalkDir::new(root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .filter(|e| {
            e.path()
                .extension()
                .map(|ext| ext == "json")
                .unwrap_or_default()
        })
        .map(|e| e.path().into())
}
