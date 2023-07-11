use std::path::{Path, PathBuf};

use eyre::Result;

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
